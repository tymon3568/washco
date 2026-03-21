# Common Patterns (CP) - WashCo Network

Tài liệu này mô tả các pattern chung mà AI agent và contributor phải tuân thủ khi làm việc với dự án WashCo.

---

## 1. Ngôn ngữ giao diện (UI Language)

### Quy tắc bắt buộc
- **Luôn dùng tiếng Việt CÓ DẤU** cho mọi text hiển thị trên giao diện (labels, buttons, messages, placeholders, tooltips, toasts, error messages).
- Không dùng tiếng Việt không dấu (ví dụ: ~~"Khach hang"~~ → "Khách hàng").
- Không dùng tiếng Anh cho UI labels trừ các thuật ngữ kỹ thuật phổ biến (VD: "OTP", "QR", "walk-in", "check-in", "User ID", "VND").

### Bảng tham chiếu từ vựng thường dùng

| Tiếng Việt có dấu | Ngữ cảnh |
|---|---|
| Đăng nhập / Đăng xuất / Đăng ký | Auth |
| Số điện thoại | Form label |
| Khách hàng | Customer |
| Nhân viên | Staff |
| Dịch vụ | Service/Catalog |
| Thanh toán | Payment |
| Hàng đợi | Queue |
| Đặt lịch | Booking |
| Khuyến mãi | Promotion |
| Đánh giá | Review |
| Báo cáo | Analytics |
| Cài đặt | Settings |
| Vật tư / Kho vật tư | Inventory |
| Hoa hồng | Commission |
| Ca làm việc | Shift |
| Thêm / Sửa / Xóa / Hủy | CRUD actions |
| Xác nhận | Confirm |
| Lưu / Lưu thay đổi | Save |
| Đang tải... / Đang xử lý... | Loading states |
| Có lỗi xảy ra | Generic error |
| Hoàn thành | Completed status |
| Đang chờ / Chờ xử lý | Pending status |
| Cửa hàng | Location/Store |
| Doanh thu | Revenue |
| Tiền mặt / Chuyển khoản / Ví điện tử | Payment methods |
| Phân khúc | Customer segment |
| Biển số | License plate |
| Xe máy / Xe tải | Vehicle types |
| Tồn kho / Nhập kho / Xuất kho | Inventory operations |
| Cảnh báo / Độ lệch | Inventory reports |
| Giờ hoạt động | Operating hours |
| Phút / Giờ / Ngày | Time units |

---

## 2. Frontend Patterns (SvelteKit 5)

### 2.1 Svelte 5 Runes (BẮT BUỘC)

```svelte
<!-- ĐÚNG -->
<script lang="ts">
  let count = $state(0);
  let doubled = $derived(count * 2);
  let { title, items = $bindable() } = $props<{ title: string; items?: string[] }>();
  $effect(() => { console.log(count); });
</script>
{@render children()}

<!-- SAI - không dùng -->
<!-- export let, $:, writable(), <slot /> -->
```

### 2.2 API Client

```typescript
import { api, ApiError } from '$lib/api/client';
import { toast } from '$lib/toast.svelte';
import type { SomeResponse } from '$lib/api/types';

// GET
const data = await api.get<SomeResponse[]>('/module/endpoint');

// POST
await api.post('/module/endpoint', { field: value });

// PUT
await api.put(`/module/${id}`, { field: value });

// DELETE
await api.del(`/module/${id}`);

// Error handling
try {
  await api.post('/module/endpoint', body);
  toast.success('Thành công');
} catch (e: unknown) {
  toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
}
```

### 2.3 Page Structure Pattern

```svelte
<script lang="ts">
  import { api, ApiError } from '$lib/api/client';
  import { toast } from '$lib/toast.svelte';
  import { formatVND } from '$lib/utils/format';
  import type { SomeResponse } from '$lib/api/types';

  let items: SomeResponse[] = $state([]);
  let locationId = $state('');
  let loading = $state(false);

  // Load location on mount
  $effect(() => {
    loadData();
  });

  async function loadData() {
    try {
      const locations = await api.get<any[]>('/locations');
      if (locations.length > 0) {
        locationId = locations[0].id;
        await refreshItems();
      }
    } catch {
      // API not available
    }
  }

  async function refreshItems() {
    if (!locationId) return;
    items = await api.get<SomeResponse[]>(`/module/locations/${locationId}/items`);
  }
</script>
```

### 2.4 Styling Conventions (Tailwind)

| Element | Classes |
|---|---|
| Card | `rounded-lg border border-border bg-card p-6 shadow-xs` |
| Table | `min-w-full divide-y divide-border` |
| Table header | `bg-muted/50`, `px-4 py-3 text-sm font-medium text-muted-foreground` |
| Table cell | `px-4 py-3 text-sm` |
| Primary button | `rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50` |
| Secondary button | `rounded-md border border-border px-4 py-2 text-sm hover:bg-muted` |
| Destructive button | `rounded-md bg-destructive/10 px-3 py-1.5 text-xs text-destructive hover:bg-destructive/20` |
| Input | `rounded-md border border-input bg-background px-3 py-2 text-sm` |
| Badge (green) | `bg-green-100 text-green-700` |
| Badge (red) | `bg-red-100 text-red-700` |
| Badge (blue) | `bg-blue-100 text-blue-700` |
| Badge (amber) | `bg-amber-100 text-amber-700` |
| Badge (gray) | `bg-gray-100 text-gray-600` |
| Page title | `text-2xl font-semibold` |
| Subtitle | `mt-1 text-sm text-muted-foreground` |
| Money display | `font-mono` + `formatVND()` |

### 2.5 Tab Navigation Pattern

```svelte
<div class="mt-6 flex gap-1 rounded-lg bg-muted p-1">
  {#each tabs as tab (tab.key)}
    <button
      onclick={() => (activeTab = tab.key)}
      class="flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors {activeTab === tab.key
        ? 'bg-background text-foreground shadow-sm'
        : 'text-muted-foreground hover:text-foreground'}"
    >
      {tab.label}
    </button>
  {/each}
</div>
```

### 2.6 TypeScript Types

Mọi API response type đều được khai báo trong `frontend/owner/src/lib/api/types.ts`. Khi thêm module mới:
1. Thêm interface cho Response và Request types
2. Import type trong page component: `import type { XxxResponse } from '$lib/api/types'`

### 2.7 Toast Messages

```typescript
toast.success('Đã thêm thành công');    // Thao tác thành công
toast.error('Có lỗi xảy ra');           // Lỗi chung
toast.error(e.message);                  // Lỗi cụ thể từ API
toast.info('Thông tin');                 // Thông báo thông tin
```

### 2.8 Money Formatting

```typescript
import { formatVND } from '$lib/utils/format';

formatVND(150000);  // "150.000đ"
formatVND(0);       // "0đ"
```

---

## 3. Backend Patterns (Rust)

### 3.1 Module Structure

```
crates/modules/<name>/
├── Cargo.toml
├── src/
│   ├── lib.rs              # pub mod api; pub(crate) mod application, domain, infra
│   ├── api/
│   │   ├── mod.rs           # XxxState + routes(pool, jwt) -> Router
│   │   ├── dto.rs           # Request/Response DTOs
│   │   └── handlers.rs      # Axum handlers
│   ├── application/
│   │   ├── mod.rs
│   │   ├── ports.rs         # Repository trait (impl Future, no async_trait)
│   │   └── services.rs      # XxxService<R: XxxRepository>
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── entities.rs      # Structs + enums with from_str() -> Option<Self>
│   │   └── errors.rs        # XxxError + From<XxxError> for AppError
│   └── infra/
│       ├── mod.rs
│       └── pg_xxx_repo.rs   # PgXxxRepository with sqlx::query() + Row::get()
```

### 3.2 State Pattern

```rust
#[derive(Clone)]
pub struct XxxState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for XxxState {
    type Target = Service;
    fn deref(&self) -> &Self::Target { &self.service }
}

impl AsRef<JwtConfig> for XxxState {
    fn as_ref(&self) -> &JwtConfig { &self.jwt }
}
```

### 3.3 Routes Function

```rust
pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgXxxRepository::new(pool);
    let service = Arc::new(XxxService::new(repo));
    let state = XxxState { service, jwt };

    Router::new()
        .route("/endpoint", get(handlers::list).post(handlers::create))
        .with_state(state)
}
```

### 3.4 SQLx Pattern (Unchecked)

```rust
// Dùng sqlx::query() + .bind() + Row::get() (không cần DATABASE_URL lúc compile)
let row = sqlx::query(
    "SELECT id, name FROM table WHERE tenant_id = $1 AND id = $2"
)
.bind(tenant_id)
.bind(id)
.fetch_one(&self.pool)
.await?;

Ok(Entity {
    id: row.get("id"),
    name: row.get("name"),
})
```

### 3.5 Domain Enum Pattern

```rust
pub enum Status {
    Active,
    Inactive,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            _ => None,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
```

### 3.6 RBAC Pattern

```rust
// Trong handler
pub async fn create(
    State(state): State<XxxState>,
    ctx: TenantContext,
    Json(body): Json<CreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    ctx.require_manager_or_above()?;  // Hoặc require_owner_or_admin(), require_role(&[...])
    // ...
}
```

### 3.7 Multi-tenancy (BẮT BUỘC)

- Mọi query tenant-scoped PHẢI filter `tenant_id`
- Repository method PHẢI nhận `tenant_id: Uuid` parameter
- JOIN phải tenant-safe

---

## 4. Infrastructure Patterns

### 4.1 API Route Registration

```rust
// crates/server/src/router.rs
.nest(
    "/api/v1/<module>",
    washco_<module>::api::routes(state.db.clone(), state.jwt.clone()),
)
```

### 4.2 Workspace Member

```toml
# Cargo.toml (root)
members = [
    "crates/modules/<name>",
]

# crates/server/Cargo.toml
washco-<name> = { path = "../modules/<name>" }
```

### 4.3 Navigation (Frontend)

Thêm vào `navItems` trong `frontend/owner/src/routes/(dashboard)/+layout.svelte`:

```typescript
{ href: '/<route>', label: '<Tên tiếng Việt có dấu>', icon: '<icon-name>' }
```

Và thêm SVG icon tương ứng trong phần mobile bottom nav.

---

## 5. Checklist khi thêm Module mới

### Backend
- [ ] Tạo crate `crates/modules/<name>/` với 4 layers
- [ ] Domain: entities + errors + enums (from_str → Option)
- [ ] Application: ports trait (impl Future) + service
- [ ] Infra: PgRepository (sqlx::query unchecked, tenant_id filter)
- [ ] API: State (Deref + AsRef) + routes + handlers (RBAC) + DTOs
- [ ] Thêm vào workspace `Cargo.toml`
- [ ] Thêm dependency trong `crates/server/Cargo.toml`
- [ ] Wire routes trong `crates/server/src/router.rs`
- [ ] `cargo check --workspace && cargo clippy -- -D warnings && cargo fmt`

### Frontend
- [ ] Thêm TypeScript types vào `frontend/owner/src/lib/api/types.ts`
- [ ] Tạo page `frontend/owner/src/routes/(dashboard)/<route>/+page.svelte`
- [ ] Thêm nav item + icon vào layout
- [ ] Dùng tiếng Việt CÓ DẤU cho mọi UI text
- [ ] Dùng Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`)
- [ ] `bun run build && bun run check`

### Database
- [ ] Migration file trong `migrations/`
- [ ] Indexes cho tenant_id + các query patterns
- [ ] Foreign keys với ON DELETE policy phù hợp
