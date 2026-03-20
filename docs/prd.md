# Product Requirements Document

## Product Name
WashCo Network

## Document Status
Draft v1.0

## 1. Executive Summary
WashCo Network is a two-sided operations and discovery platform that connects car wash business owners and drivers across Vietnam. The product helps owners run their locations more efficiently while helping drivers find trusted, available, and fairly priced car wash services nearby.

The platform focuses on three core outcomes:

1. Increase utilization and revenue for car wash locations.
2. Reduce wait time, idle labor, and operational inefficiencies.
3. Improve convenience, transparency, and repeat usage for drivers.

## 2. Problem Statement
Car wash operations in Vietnam are highly fragmented. Most sites still rely on manual processes for queue management, staffing, pricing, customer communication, and daily reporting. Drivers often struggle to know which locations are open, how long the wait will be, how much a service will cost, and whether the quality is reliable.

This creates several problems:

- Owners cannot predict demand accurately or optimize labor and bay utilization.
- Staff scheduling is reactive and inconsistent.
- Drivers waste time traveling to crowded or low-quality locations.
- Pricing and service packages are unclear or inconsistent.
- Repeat customer retention is weak due to poor CRM and limited loyalty systems.
- Multi-location operators lack standardized performance visibility.

## 3. Vision
Build the operating system for car wash networks in Vietnam by combining location discovery, real-time operations management, and customer engagement in one platform.

## 4. Product Goals

### Business Goals
- Onboard a critical mass of car wash locations in major Vietnamese cities.
- Increase monthly gross transaction volume across participating locations.
- Build recurring software revenue from owner-facing operations tools.
- Create monetization upside from demand generation and premium visibility.
- Improve owner retention by making the platform central to daily operations.
- Establish a scalable data foundation for pricing, demand forecasting, and expansion.

### User Goals
- Owners want higher throughput, more predictable demand, and better control of operations.
- Drivers want fast discovery, transparent pricing, shorter waits, and reliable quality.

## 5. Business Model and Monetization

WashCo Network should be designed as a hybrid SaaS plus marketplace business, with monetization phased to match market maturity.

### Primary Revenue Streams
- SaaS subscription from owners for operations software such as queue management, reporting, CRM, and promotions.
- Commission on completed bookings or digital transactions generated through WashCo demand channels.
- Promoted listings and sponsored campaigns for locations that want higher visibility in search and map results.

### Suggested Monetization Rollout
- Phase 1: Free or low-cost owner SaaS to accelerate supply acquisition and habituate operators to daily usage.
- Phase 2: Introduce marketplace commission for bookings generated through WashCo driver channels.
- Phase 3: Add premium subscription tiers, promoted placement, and advanced analytics for multi-site operators.

### Monetization Principles
- Monetization should not block owner onboarding during the supply acquisition stage.
- Pricing should align with measurable value such as increased bookings, better utilization, and time saved.
- Revenue mix should reduce dependence on a single monetization stream.

## 6. Target Users

### Primary User: Car Wash Owner / Operator
Includes independent shop owners, site managers, and small regional chains.

Needs:
- See bookings, walk-ins, queue length, and staff status in one place.
- Manage service menus, prices, promotions, and operating hours.
- Improve labor productivity and daily revenue.
- Understand which services, time slots, and customer segments perform best.

### Secondary User: Driver
Includes private car owners, ride-hailing drivers, taxi drivers, fleet drivers, and delivery vehicle operators.

Needs:
- Find a nearby trusted location quickly.
- Compare price, services, ratings, and wait times.
- Book or join a queue remotely.
- Receive a consistent and convenient service experience.

### Tertiary User: Network Admin / Franchise Manager
Needs:
- Monitor multi-location performance.
- Standardize pricing, promotions, and service quality.
- Track compliance and performance across locations.

## 7. Market Context
Vietnam has dense urban traffic, high motorbike and growing car ownership, fragmented service quality, and strong mobile-first behavior. Drivers value convenience, price transparency, and speed. Owners need practical tools that fit local operations and can work with limited digital maturity.

The product should be designed for:
- Ho Chi Minh City and Hanoi as initial launch markets.
- Vietnamese-language operations in the product UI, even if this document is in English.
- Local payment preferences such as cash, bank transfer, QR payment, and e-wallets where applicable.
- Mobile-first usage for both drivers and site staff.
- Low-friction consumer acquisition through channels Vietnamese users already trust and use daily.

## 8. Jobs To Be Done

### Owner Jobs
- When demand changes by hour and day, help me forecast workload so I can staff correctly.
- When customers arrive or book online, help me manage queue flow so bays are utilized efficiently.
- When service is completed, help me collect payment and record performance without manual paperwork.
- When I run promotions, help me attract nearby drivers during low-demand periods.

### Driver Jobs
- When my vehicle needs cleaning, help me find the best nearby location with confidence.
- When I am busy, help me reserve a slot or estimate wait time before I arrive.
- When I pay, help me understand exactly what I am paying for.
- When I find a good location, help me return easily and receive relevant offers.

## 9. Product Scope

### In Scope for MVP
- Driver-facing Zalo Mini App for discovery, booking, queueing, loyalty, and post-service reviews.
- Mobile web fallback for store profile sharing and lightweight discovery when needed.
- Owner dashboard for daily operations.
- Real-time store profile and service catalog.
- Queue and booking management.
- Basic pricing and promotion management.
- Ratings and review capture.
- Basic reporting and analytics.
- Notification system for booking status and promotions.
- QR-based entry points at partner stores to open the driver experience with minimal friction.

### Out of Scope for MVP
- Full ERP or accounting suite.
- Advanced AI-based camera inspection.
- Full fleet contract procurement workflows.
- Hardware integration beyond simple optional check-in tools.
- Native consumer app as the primary driver channel.

### MVP Platform Strategy
- Owner operations should be delivered through a web dashboard optimized for mobile and desktop usage.
- Driver acquisition should prioritize Zalo Mini App because it reduces install friction, supports QR-led in-store journeys, and fits user behavior in Vietnam.
- Native driver apps may be considered later only after repeat usage justifies the higher acquisition and retention cost.

## 10. Core Value Propositions

### For Owners
- Fill more bays during off-peak hours.
- Reduce manual coordination and phone-based booking.
- Standardize service and pricing presentation.
- Use data to improve staffing, service mix, and promotions.

### For Drivers
- Discover trusted car wash locations nearby.
- See transparent pricing and estimated wait time.
- Book ahead or join a digital queue.
- Get a faster, more predictable experience.

## 11. Key Product Principles
- Mobile-first, low-friction, operationally practical.
- Transparent information for both sides.
- Real-time visibility where it changes decisions.
- Localized for Vietnam market realities.
- Designed to scale from single-site operators to chains.
- Bias toward low-CAC growth loops such as Zalo sharing, QR entry, and owner-led distribution.

## 12. MVP Feature Requirements

### 12.1 Driver Experience

#### Location Discovery
- Show nearby car wash locations on map and list view.
- Filter by distance, open now, price range, service type, rating, and wait time.
- Display store profile with photos, services, prices, hours, amenities, and reviews.
- Allow drivers to enter via Zalo Mini App from QR codes, search, shared links, or official account messages.

#### Booking and Queue
- Allow drivers to book a time slot where supported.
- Allow drivers to join a live queue for walk-in service.
- Show estimated start time and live queue updates.
- Allow cancellation or rescheduling within policy limits.
- Show late-arrival and no-show policy before booking confirmation.

#### Payments and Offers
- Support cash and mark additional supported methods per location.
- Show price estimates before booking.
- Support promo codes or location-based discounts.
- Support weather-triggered or off-peak campaigns when applicable.

#### Trust and Retention
- Ratings and reviews after service.
- Favorite locations and rebook flow.
- Booking history and receipts.
- QR-based loyalty, post-service review prompts, and repeat-visit offers.

### 12.2 Owner Experience

#### Location Setup
- Create and edit store profile.
- Configure hours, address, service areas, service menu, prices, and photos.
- Define bay capacity, average service duration, and supported payment methods.

#### Daily Operations
- View current queue, bookings, walk-ins, and job status.
- Assign jobs to bays or staff.
- Mark stages such as checked-in, washing, detailing, completed, paid.
- Adjust estimated wait times manually when needed.

#### Pricing and Promotions
- Update service prices.
- Create promotions by date, time window, service type, or customer segment.
- Launch off-peak campaigns to attract nearby drivers.
- Configure booking grace periods, deposit rules for premium services, and no-show policies.

#### Reporting
- Daily revenue summary.
- Booking conversion, walk-in volume, average wait time, utilization rate.
- Top services, repeat customers, cancellation rate.
- No-show, late-arrival, and promotion performance reporting.

### 12.3 Admin Experience
- Approve and manage locations.
- Monitor marketplace quality and service compliance.
- Review core metrics by city, district, and operator.

## 13. Functional Requirements

### Account and Identity
- Drivers can start through Zalo Mini App and complete phone number OTP verification for booking, receipts, and loyalty flows.
- Owners can sign up and submit business information for verification.
- Role-based access for owner, manager, cashier, and staff.

### Search and Ranking
- Search results should prioritize relevance based on proximity, availability, quality score, and user preferences.
- Ranking logic should be configurable by city and campaign rules.

### Queue Management
- Owners can configure queue mode: bookings only, walk-ins only, or hybrid.
- System should estimate wait time using active jobs, bay capacity, and average service duration.
- Drivers should see queue status refresh in near real time.

### Booking System
- Time slots should reflect capacity constraints.
- Prevent overbooking based on active bay and staffing limits.
- Support service add-ons during booking or check-in.
- Auto-release a reserved slot after a configurable grace period, with a default recommended range of 10 to 15 minutes after scheduled arrival time.
- Track late arrivals and no-shows at the user level.
- Restrict booking privileges after repeated no-shows based on configurable thresholds.
- Support optional deposits for high-value services such as detailing or premium packages.
- Backfill released or canceled slots with walk-ins or waitlisted demand where applicable.

### Pricing Engine
- Store base price by service type and vehicle type.
- Support temporary promotional pricing.
- Show final estimated price before confirmation.

### Notifications
- Send confirmations, reminders, queue updates, and completion notices.
- Support push notifications, SMS fallback, and in-app messaging.
- Support Zalo-based re-engagement and post-service prompts where channel policy allows.

### Reviews
- Only verified completed visits can leave ratings and reviews.
- Owners can respond to reviews.

### Analytics
- Owners can view trends by day, week, and month.
- Admins can compare location performance and identify underperforming sites.

## 14. Optimization Capabilities
The product should not only connect supply and demand, but actively optimize operations.

### Demand Optimization
- Predict high-demand windows from historical data.
- Recommend promotions during slow periods.
- Recommend extended hours during peak periods when profitable.

### Capacity Optimization
- Estimate bay utilization and identify bottlenecks.
- Recommend staffing levels by daypart.
- Highlight locations with repeated over-capacity or under-capacity issues.

### Service Mix Optimization
- Show which services generate the highest margin and fastest throughput.
- Recommend bundles by customer segment and time of day.

### Geo Optimization
- Surface nearby demand opportunities to participating locations.
- Recommend the best store for a driver based on travel time, queue, and preferences.

### Weather-Aware Optimization
- Incorporate weather data and forecast signals into demand prediction.
- Recommend rain-day or low-demand promotions automatically or semi-automatically.
- Alert owners when upcoming weather is likely to impact traffic so they can adjust staffing and promotions.
- Use weather patterns as an explanatory factor in reporting and forecasting.

## 15. User Flows

### Driver Flow
1. Open Zalo Mini App from QR code, search, shared link, or official account entry point.
2. View nearby car wash locations.
3. Compare options by price, rating, distance, and wait time.
4. Select service and join queue or book slot.
5. Review booking rules, including late-arrival, cancellation, and deposit policies where applicable.
6. Receive confirmation and arrival instructions.
7. Check in at location.
8. Receive status updates.
9. Pay and leave review.

### Owner Flow
1. Register business and complete verification.
2. Configure location profile, services, capacity, and pricing.
3. Receive bookings and monitor walk-ins.
4. Manage queue and service progress.
5. Launch targeted promotions during low-demand periods.
6. Review daily operations and performance metrics.

## 16. Success Metrics

### Marketplace Metrics
- Number of active locations.
- Number of monthly active drivers.
- Booking volume and completed visits.
- Gross merchandise value or total transaction value.
- Marketplace commission revenue.
- Paid owner subscription count and monthly recurring revenue.

### Operations Metrics
- Average wait time.
- Bay utilization rate.
- Booking fill rate.
- Staff productivity per shift.
- Cancellation and no-show rate.

### Experience Metrics
- Driver repeat rate.
- Owner retention rate.
- Net Promoter Score or satisfaction score.
- Review volume and average rating.

## 17. Non-Functional Requirements
- Fast mobile performance on mid-range Android devices.
- High availability during peak traffic windows.
- Secure user authentication and role-based access control.
- Audit logs for pricing, booking, and operational changes.
- Privacy-compliant handling of phone numbers, payment data, and location data.
- Scalable architecture for multi-city expansion.
- Reliable QR-to-experience flow for in-store driver onboarding.
- Graceful degradation for intermittent connectivity at car wash locations.
- Channel-aware architecture that can support Zalo Mini App as the primary consumer surface in MVP.

## 18. Localization Requirements
- Support Vietnamese as the primary product language for launch.
- Use local address formats and map behavior suitable for Vietnam.
- Support VND pricing everywhere.
- Design for district-level operational analysis in major cities.

## 19. Risks and Constraints
- Owners may resist operational changes if setup feels complex.
- Wait-time accuracy may be poor early without enough usage data.
- Marketplace quality can decline without location verification and review controls.
- Payment fragmentation may require phased rollout.
- Operational discipline at locations will affect service consistency.
- Dependence on a third-party channel such as Zalo can create platform and policy risk.
- Poor handling of no-shows or late arrivals can damage owner trust in booking workflows.
- Weather volatility can create sharp demand swings that weaken forecast accuracy in early stages.

## 20. Assumptions
- Car wash owners are willing to adopt free or low-cost operations software if setup is simple and immediate value is visible.
- Drivers are more likely to engage through low-friction channels such as Zalo Mini App than through a standalone app in the early stage.
- Initial city density will be sufficient to create marketplace utility once a strong enough supply base is onboarded.

## 21. Product and Go-To-Market Launch Plan

### GTM Strategy
- Solve the marketplace chicken-and-egg problem by acquiring the hard side first: car wash owners.
- Use owner software as the initial wedge, even before marketplace demand is fully active.
- Treat the driver experience as a demand layer that scales after sufficient supply density and service quality are in place.

### Phase 1
- Launch free or low-cost owner onboarding, store profiles, and daily operations tools for managing walk-ins.
- Focus on queue visibility, pricing setup, reporting, and operational habit formation at partner locations.
- Seed QR-based customer review and loyalty entry points at stores.

### Phase 2
- Launch driver-facing Zalo Mini App discovery, store detail, reviews, and lightweight booking or queue joining.
- Launch basic reporting, promotions, and weather-triggered campaigns.
- Begin charging commission on marketplace-generated demand where value is proven.

### Phase 3
- Launch optimization recommendations and multi-location analytics.
- Introduce premium SaaS tiers, sponsored placement, and more advanced demand-routing logic.

## 22. Future Opportunities
- Fleet account management for taxi and logistics partners.
- Membership and subscription wash plans.
- Dynamic pricing based on demand and capacity.
- CRM automation and loyalty rewards.
- Staff incentive tracking.
- Add-on services such as detailing, oil change, or maintenance referrals.

## 23. Open Questions
- Should the first release prioritize car owners only, or also support motorbike wash locations?
- Should booking be mandatory for some premium locations or optional across the network?
- Which notification channels will be most reliable and cost-effective at launch?
- What level of owner verification is required before a location becomes visible to drivers?

## 24. Recommended MVP Statement
Build an owner-first, mobile-first platform for Vietnam that gives car wash operators practical software to manage daily operations and then layers on a low-friction driver experience through Zalo Mini App to drive discovery, bookings, loyalty, and better capacity utilization.
