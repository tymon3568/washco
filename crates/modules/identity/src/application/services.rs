use uuid::Uuid;
use washco_shared::{AppError, JwtConfig};

use crate::domain::{IdentityError, OtpEntry, Tenant, User};

use super::ports::{OtpStore, UserRepository};

pub struct IdentityService<R, O> {
    repo: R,
    otp_store: O,
    jwt: JwtConfig,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct RegisterInput {
    pub phone: String,
    pub business_name: String,
    pub owner_name: String,
}

impl<R: UserRepository, O: OtpStore> IdentityService<R, O> {
    pub fn new(repo: R, otp_store: O, jwt: JwtConfig) -> Self {
        Self {
            repo,
            otp_store,
            jwt,
        }
    }

    pub async fn register(&self, input: RegisterInput) -> Result<User, AppError> {
        if self.repo.find_by_phone(&input.phone).await?.is_some() {
            return Err(IdentityError::PhoneAlreadyExists.into());
        }

        let tenant = Tenant::new(input.business_name, input.owner_name.clone());
        self.repo.create_tenant(&tenant).await?;

        let user = User::new_owner(tenant.id, input.phone, input.owner_name);
        self.repo.create_user(&user).await?;

        Ok(user)
    }

    pub async fn request_otp(&self, phone: &str) -> Result<(), AppError> {
        self.repo
            .find_by_phone(phone)
            .await?
            .ok_or(IdentityError::UserNotFound)?;

        let otp = OtpEntry::new(phone.to_string());
        tracing::info!(phone = phone, code = %otp.code, "OTP generated (dev mode)");
        self.otp_store.store(otp).await?;

        Ok(())
    }

    pub async fn verify_otp_and_login(
        &self,
        phone: &str,
        code: &str,
    ) -> Result<TokenPair, AppError> {
        let otp = self
            .otp_store
            .get(phone)
            .await?
            .ok_or(IdentityError::NoOtpRequest)?;

        if otp.is_expired() {
            self.otp_store.remove(phone).await?;
            return Err(IdentityError::OtpExpired.into());
        }

        // Dev mode: accept "000000" as universal OTP
        let is_dev_bypass = cfg!(debug_assertions) && code == "000000";
        if !is_dev_bypass && !otp.matches(code) {
            return Err(IdentityError::InvalidOtp.into());
        }

        self.otp_store.remove(phone).await?;

        let mut user = self
            .repo
            .find_by_phone(phone)
            .await?
            .ok_or(IdentityError::UserNotFound)?;

        if !user.is_verified {
            user.verify();
            self.repo.update_user(&user).await?;
        }

        self.generate_tokens(&user).await
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        self.repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| IdentityError::UserNotFound.into())
    }

    pub fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, AppError> {
        let claims = self.jwt.verify_token(refresh_token)?;
        let access =
            self.jwt
                .generate_access_token(claims.sub, claims.tenant_id, claims.role.clone())?;
        let refresh = self
            .jwt
            .generate_refresh_token(claims.sub, claims.tenant_id, claims.role)?;
        Ok(TokenPair {
            access_token: access,
            refresh_token: refresh,
        })
    }

    async fn generate_tokens(&self, user: &User) -> Result<TokenPair, AppError> {
        let (tier, features) = self
            .repo
            .find_tenant_tier_features(user.tenant_id)
            .await
            .unwrap_or((None, vec![]));

        let access = self.jwt.generate_access_token_with_tier(
            user.id,
            user.tenant_id,
            user.role.clone(),
            tier,
            features,
        )?;
        let refresh =
            self.jwt
                .generate_refresh_token(user.id, user.tenant_id, user.role.clone())?;
        Ok(TokenPair {
            access_token: access,
            refresh_token: refresh,
        })
    }
}
