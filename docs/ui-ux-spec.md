# WashCo Network - UI/UX Specification

## 1. Design System Foundation

### 1.1 Component Library

**Base**: shadcn-svelte (copy-paste, Svelte 5 runes, Bits UI primitives)
**Customization**: WashCo theme layer via CSS variables
**Styling**: Tailwind CSS 4

shadcn-svelte is not installed as a dependency. Components are copied into `$lib/components/ui/`
and fully owned by the project. This gives complete control while starting from accessible,
well-tested primitives.

### 1.2 Color System

Inspired by luma.com's refined, neutral-first palette with a distinctive brand accent.

```css
:root {
  /* --- Neutral (luma-inspired: warm gray, not cold) --- */
  --background:        0 0% 100%;        /* #FFFFFF */
  --foreground:        240 10% 3.9%;     /* #0A0A0B - near black, warm */
  --muted:             240 4.8% 95.9%;   /* #F4F4F5 */
  --muted-foreground:  240 3.8% 46.1%;   /* #71717A */
  --card:              0 0% 100%;
  --card-foreground:   240 10% 3.9%;
  --border:            240 5.9% 90%;     /* #E4E4E7 */
  --input:             240 5.9% 90%;
  --ring:              221 83% 53%;      /* focus ring: brand blue */

  /* --- Brand: WashCo Blue (trust, clean, water) --- */
  --primary:           221 83% 53%;      /* #3B82F6 */
  --primary-foreground: 0 0% 100%;

  /* --- Secondary: Soft slate --- */
  --secondary:         240 4.8% 95.9%;
  --secondary-foreground: 240 5.9% 10%;

  /* --- Accent: Warm teal (energy, freshness) --- */
  --accent:            172 66% 50%;      /* #22B8A0 */
  --accent-foreground: 0 0% 100%;

  /* --- Semantic --- */
  --destructive:       0 84% 60%;        /* #EF4444 */
  --destructive-foreground: 0 0% 100%;
  --success:           142 71% 45%;      /* #22C55E */
  --success-foreground: 0 0% 100%;
  --warning:           38 92% 50%;       /* #F59E0B */
  --warning-foreground: 0 0% 100%;
  --info:              221 83% 53%;      /* same as primary */
  --info-foreground:   0 0% 100%;

  /* --- Radius --- */
  --radius:            0.5rem;           /* consistent rounded corners */
}

/* Dark mode (owner dashboard, optional) */
.dark {
  --background:        240 10% 3.9%;
  --foreground:        0 0% 98%;
  --muted:             240 3.7% 15.9%;
  --muted-foreground:  240 5% 64.9%;
  --card:              240 10% 3.9%;
  --card-foreground:   0 0% 98%;
  --border:            240 3.7% 15.9%;
  --input:             240 3.7% 15.9%;
  --primary:           217 91% 60%;
  --primary-foreground: 0 0% 100%;
  --accent:            172 66% 50%;
  --accent-foreground: 0 0% 100%;
}
```

Color usage rules:
- `primary` (blue): CTAs, links, active states, selected items
- `accent` (teal): success indicators, positive metrics, available status
- `destructive` (red): errors, delete actions, critical alerts
- `warning` (amber): queue warnings, capacity alerts, expiring promos
- `muted`: backgrounds, disabled states, secondary text
- Neutrals carry 80% of the UI. Color is used sparingly for meaning.

### 1.3 Typography

```css
:root {
  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
}
```

| Level     | Size  | Weight | Line Height | Usage                       |
|-----------|-------|--------|-------------|-----------------------------|
| Display   | 30px  | 700    | 1.2         | Page titles (dashboard)     |
| Heading 1 | 24px  | 600    | 1.3         | Section headers             |
| Heading 2 | 20px  | 600    | 1.3         | Card titles                 |
| Heading 3 | 16px  | 600    | 1.4         | Sub-sections                |
| Body      | 14px  | 400    | 1.5         | Default text                |
| Small     | 12px  | 400    | 1.5         | Captions, timestamps        |
| Mono      | 13px  | 400    | 1.5         | Prices, IDs, codes          |

Vietnamese text note: Inter supports Vietnamese diacritics. All text must render
correctly with Vietnamese characters (e.g. "Tiệm rửa xe", "Đặt lịch", "Đánh giá").

### 1.4 Spacing & Layout

8px grid system:
- `xs`: 4px
- `sm`: 8px
- `md`: 16px
- `lg`: 24px
- `xl`: 32px
- `2xl`: 48px

Content max-width: `1280px` for dashboard, `640px` for driver mobile web.

### 1.5 Shadows & Elevation

```css
--shadow-xs:  0 1px 2px 0 rgb(0 0 0 / 0.05);
--shadow-sm:  0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
--shadow-md:  0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
--shadow-lg:  0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
```

Usage: cards use `shadow-xs` at rest, `shadow-sm` on hover. Dialogs/popovers use `shadow-lg`.

## 2. Component Inventory

### 2.1 shadcn-svelte Components (from library)

Core layout:
- `Card` - primary container for all data sections
- `Separator` - visual dividers
- `Sheet` - slide-over panels (mobile nav, filters)
- `Tabs` - section switching within a page
- `ScrollArea` - scrollable containers

Data display:
- `Table` - booking lists, service catalogs, analytics
- `Badge` - status indicators (queue position, booking state)
- `Avatar` - user/location photos
- `Tooltip` - contextual help

Forms & Input:
- `Button` - primary, secondary, outline, ghost, destructive variants
- `Input` - text, phone number, search
- `Textarea` - reviews, descriptions
- `Select` - dropdowns (service type, vehicle type, city)
- `Checkbox` / `Switch` - toggles (amenities, settings)
- `RadioGroup` - queue mode selection
- `DatePicker` - booking date selection
- `Slider` - price range filter
- `Form` with validation (superforms + valibot)

Feedback:
- `Alert` - inline messages
- `Sonner` (toast) - action confirmations
- `Dialog` - confirmations, destructive actions
- `AlertDialog` - critical confirmations (cancel booking, delete)
- `Skeleton` - loading states

Navigation:
- `Command` - command palette / search (owner dashboard)
- `DropdownMenu` - user menu, actions
- `Breadcrumb` - page hierarchy
- `Pagination` - list navigation

### 2.2 Custom Components (WashCo-specific)

These are built on top of shadcn primitives:

```
$lib/components/
  ui/                          # shadcn-svelte copies (owned)
    button/
    card/
    table/
    ...

  washco/                      # WashCo domain components
    QueueBoard.svelte          # Kanban-style queue visualization
    QueuePosition.svelte       # Driver's position in queue + ETA
    ServiceCard.svelte         # Service item with price + duration
    LocationCard.svelte        # Location preview (map list item)
    LocationMap.svelte         # Map view with location pins
    BookingSlotPicker.svelte   # Time slot grid for booking
    StatusBadge.svelte         # Semantic status (waiting/washing/done)
    PriceDisplay.svelte        # VND formatted price
    RatingStars.svelte         # Star rating display + input
    MetricCard.svelte          # KPI card (revenue, utilization, etc.)
    OperatingHours.svelte      # Weekly hours editor
    BayAssignment.svelte       # Drag-assign jobs to bays
    PromoBuilder.svelte        # Promotion creation form
    StatsChart.svelte          # Lightweight chart wrapper
```

## 3. Owner Dashboard UI

### 3.1 Layout Structure

```
+----------------------------------------------------------+
| TopBar: Logo | Search (Cmd+K) | Notifications | UserMenu |
+--------+-----------------------------------------------------+
| Sidebar|                                                  |
|        |  Breadcrumb: Dashboard > Queue                   |
| Dash   |                                                  |
| Queue  |  +------------------+  +------------------+     |
| Book   |  | MetricCard       |  | MetricCard       |     |
| Catalog|  | Today Revenue    |  | Queue Length      |     |
| Promo  |  | 4,250,000d       |  | 3 waiting         |     |
| Report |  +------------------+  +------------------+     |
| Setting|                                                  |
|        |  +--------------------------------------------+  |
|        |  | Main Content Area                          |  |
|        |  |                                            |  |
|        |  +--------------------------------------------+  |
+--------+--------------------------------------------------+
```

Sidebar: collapsible on mobile, icon-only on tablet, full on desktop.
TopBar: fixed, 56px height.

### 3.2 Key Pages

#### Dashboard (/)
- 4 MetricCards: today revenue, completed jobs, queue length, avg wait time
- Today's booking timeline (horizontal bar chart)
- Recent activity feed (last 10 events)
- Quick actions: "Add walk-in", "Create promo"

#### Queue Management (/queue)
**Primary owner screen - used all day**

```
+--Queue Board (Kanban)------------------------------------+
|                                                           |
|  Waiting (3)    |  In Progress (2)  |  Completed (5)     |
|  +-----------+  |  +-----------+    |  +-----------+     |
|  | #12 Nguyen|  |  | #10 Tran |    |  | #08 Le    |     |
|  | Sedan     |  |  | SUV      |    |  | Sedan     |     |
|  | Full wash |  |  | Bay 1    |    |  | 350,000d  |     |
|  | 15 min    |  |  | 20 min   |    |  | 10:30     |     |
|  +-----------+  |  +-----------+    |  +-----------+     |
|  +-----------+  |  +-----------+    |                    |
|  | #13 Pham  |  |  | #11 Vo   |    |                    |
|  | Motorbike |  |  | Sedan    |    |                    |
|  | Basic     |  |  | Bay 2    |    |                    |
|  +-----------+  |  +-----------+    |                    |
+----------------------------------------------------------+
```

- Drag-and-drop between columns (or tap to advance on mobile)
- Each card shows: queue number, customer name, vehicle type, service, estimated time
- "Add walk-in" floating action button
- Sound notification on new queue entry
- Auto-refresh via WebSocket

#### Bookings (/bookings)
- Table view: date, time, customer, service, vehicle, status, actions
- Filter by: date range, status (confirmed/completed/cancelled/no-show)
- Status badges with semantic colors
- Click row to open detail sheet

#### Catalog (/catalog)
- Service list with inline editing
- Fields: name (vi), vehicle type, base price (VND), duration, description
- Drag to reorder
- Toggle service active/inactive

#### Location Settings (/settings/location)
- Store profile form: name, address, phone, photos
- Operating hours editor (weekly grid)
- Bay configuration (count, names)
- Queue mode selector (booking only / walk-in only / hybrid)
- Payment methods checkboxes
- Amenities toggles (wifi, waiting area, drinks, etc.)

#### Analytics (/analytics)
- Date range selector (today, 7d, 30d, custom)
- KPI row: revenue, jobs completed, avg wait, utilization rate, repeat rate
- Charts: revenue by day, jobs by hour, top services (bar), utilization (line)
- Table: daily breakdown with drill-down

#### Promotions (/promotions)
- Promotion list with status (draft/active/expired)
- Create flow: name, discount type (% or fixed), conditions, date range, time window
- Preview card showing how drivers will see it

### 3.3 Mobile Adaptation (Owner)

Owners often use phones on-site:
- Sidebar collapses to bottom tab bar (5 icons: Dashboard, Queue, Bookings, Catalog, More)
- Queue board becomes vertical stack (swipe between columns)
- Tables become card lists
- Forms use full-screen sheets
- "Add walk-in" is always accessible via FAB

## 4. Driver Web UI

### 4.1 Layout (Mobile-first, 640px max)

```
+----------------------------------+
| TopBar: Back | Title | Menu      |
+----------------------------------+
|                                  |
|  Content (scrollable)            |
|                                  |
|                                  |
+----------------------------------+
| BottomNav: Home | Search | Mine  |
+----------------------------------+
```

No sidebar. Bottom navigation for top-level sections.
Minimal chrome, maximum content.

### 4.2 Key Screens

#### Discovery (/)
```
+----------------------------------+
| Search bar + Filter icon         |
+----------------------------------+
| Map view (half screen)           |
|   [pin] [pin]    [pin]          |
|              [pin]               |
+------ toggle: Map / List --------+
| LocationCard                     |
| +-----+------------------------+|
| |photo| Sparkle Car Wash       ||
| |     | 4.5* (128) - 1.2km     ||
| |     | 80,000d+ | Open | 5min ||
| +-----+------------------------+|
| LocationCard                     |
| +-----+------------------------+|
| |photo| Pro Wash Q7            ||
| |     | 4.2* (86) - 2.4km      ||
| |     | 120,000d+ | Open | 15m ||
| +-----+------------------------+|
+----------------------------------+
```

- Map/list toggle
- Filter sheet: distance, price range, open now, rating, service type
- Sort: nearest, highest rated, shortest wait
- Pull to refresh

#### Location Detail (/location/:slug)
```
+----------------------------------+
| Photo carousel (swipeable)       |
+----------------------------------+
| Sparkle Car Wash                 |
| 4.5 ***** (128 reviews)         |
| 1.2km | Q. 1, TP. HCM           |
| Open until 20:00                 |
+----------------------------------+
| Queue: 3 waiting | ~15 min      |
+----------------------------------+
| Tabs: Services | Reviews | Info  |
+----------------------------------+
| ServiceCard                      |
| Rua xe co ban     80,000d  30m  |
| Rua xe day du    150,000d  45m  |
| Detailing        350,000d  90m  |
+----------------------------------+
| [  Dat lich  ] [ Xep hang ]     |
+----------------------------------+
```

- Sticky bottom CTA buttons
- Services tab: list with price + duration
- Reviews tab: rating distribution + review cards
- Info tab: hours, amenities, address, map, directions link

#### Booking Flow (/booking/:locationId)
Step 1: Select service + vehicle type
Step 2: Pick date + time slot (grid, unavailable slots grayed)
Step 3: Review summary (service, time, price, policies)
Step 4: Confirm -> success screen with booking code

```
+----------------------------------+
| Step 2/4: Chon thoi gian        |
+----------------------------------+
| < Thu 5, 20/03 >                |
+----------------------------------+
| 08:00  08:30  [09:00] 09:30    |
| 10:00  10:30  11:00   11:30    |
| 13:00  13:30  14:00   --:--    |
| 14:30  15:00  15:30   16:00    |
+----------------------------------+
| Selected: 09:00 - 09:30         |
| Rua xe day du | 150,000d        |
+----------------------------------+
| [ Tiep tuc ]                    |
+----------------------------------+
```

#### Queue Status (/queue/:id)
```
+----------------------------------+
| Sparkle Car Wash                 |
+----------------------------------+
|                                  |
|        Vi tri cua ban            |
|            #3                    |
|     Thoi gian cho: ~15 phut     |
|                                  |
+----------------------------------+
| Progress:                        |
| [===========            ] 35%   |
|                                  |
| #1 Dang rua  |  #2 Cho  | #3   |
+----------------------------------+
| [ Huy xep hang ]                |
+----------------------------------+
```

- Real-time updates via WebSocket
- Push notification when it's almost your turn
- Animated progress indicator

#### My Bookings (/bookings)
- Tabs: Upcoming | Completed | Cancelled
- Booking cards with status badge, location, service, date/time
- Tap to view detail, cancel, or rebook

#### Post-Service Review
- Star rating (1-5, tap)
- Optional text review
- Quick tags: "Nhanh", "Sach", "Than thien", "Gia tot"

### 4.3 Driver Web Visual Style

While using the same design tokens as owner dashboard, driver web has:
- Larger touch targets (min 44px)
- More generous spacing
- Bolder typography for key info (price, wait time)
- More use of `accent` color for positive states (available, short wait)
- Photo-forward layout for location cards
- Bottom sheet patterns instead of dialogs for mobile

## 5. Status System (Shared)

Consistent status colors across all surfaces:

| Status       | Color         | Badge Variant  | Context                    |
|--------------|---------------|----------------|----------------------------|
| Pending      | `muted`       | outline        | Booking awaiting confirm   |
| Confirmed    | `primary`     | default        | Booking confirmed          |
| Waiting      | `warning`     | default        | In queue, not started      |
| In Progress  | `primary`     | default        | Currently being serviced   |
| Completed    | `success`     | default        | Service done               |
| Cancelled    | `destructive` | outline        | Cancelled by user/owner    |
| No-show      | `destructive` | default        | Did not arrive             |
| Available    | `success`     | default        | Slot/bay available         |
| Full         | `destructive` | outline        | No capacity                |

## 6. Responsive Breakpoints

| Breakpoint | Width    | Target                         |
|------------|----------|--------------------------------|
| `sm`       | 640px    | Driver web max-width           |
| `md`       | 768px    | Owner mobile (tab bar nav)     |
| `lg`       | 1024px   | Owner tablet (collapsed sidebar)|
| `xl`       | 1280px   | Owner desktop (full sidebar)   |

## 7. Loading & Empty States

### Loading
- Use `Skeleton` components matching the shape of content
- Queue board: skeleton cards in each column
- Tables: skeleton rows (5 rows)
- Never show a blank page

### Empty States
- Centered illustration (simple SVG) + message + CTA
- Examples:
  - No bookings: "Chua co dat lich nao. Chia se trang cua hang de bat dau!"
  - Empty queue: "Hang doi trong. San sang phuc vu!"
  - No reviews: "Chua co danh gia. Khach hang se thay muc nay sau khi su dung dich vu."

## 8. Notifications UI

### Owner Dashboard
- Bell icon in topbar with unread count badge
- Dropdown panel showing recent notifications
- Types: new booking, new walk-in, review received, queue alert
- Click to navigate to relevant page

### Driver Web
- In-app toast for real-time updates (queue position change)
- Booking confirmation screen with "Add to calendar" option
- Push notification permission prompt (non-intrusive, after first booking)

## 9. Accessibility

- All shadcn-svelte components include WAI-ARIA by default
- Color contrast: minimum 4.5:1 for text, 3:1 for large text
- Focus ring visible on keyboard navigation (`--ring` color)
- Touch targets: minimum 44x44px on mobile
- Screen reader labels for all icon-only buttons
- Vietnamese language in all aria-labels and alt text

## 10. VND Price Formatting

All prices displayed in VND follow Vietnamese conventions:

```
150.000d        # standard display
1.250.000d      # thousands separator: dot
Tu 80.000d      # "from" prefix for range
80.000 - 150.000d  # price range
Mien phi        # free
```

Use `PriceDisplay.svelte` component everywhere for consistency.
Never show decimal places for VND (dong has no minor unit in practice).

## 11. Component File Structure

```
frontend/owner/src/lib/
  components/
    ui/                        # shadcn-svelte (copied, owned)
      button/
        button.svelte
        index.ts
      card/
      table/
      badge/
      dialog/
      input/
      select/
      tabs/
      sheet/
      sonner/
      skeleton/
      command/
      dropdown-menu/
      alert/
      avatar/
      breadcrumb/
      pagination/
      separator/
      scroll-area/
      switch/
      checkbox/
      radio-group/
      tooltip/
      alert-dialog/
      ...

    washco/                    # Domain-specific components
      QueueBoard.svelte
      QueueCard.svelte
      QueuePosition.svelte
      ServiceCard.svelte
      LocationCard.svelte
      LocationMap.svelte
      BookingSlotPicker.svelte
      StatusBadge.svelte
      PriceDisplay.svelte
      RatingStars.svelte
      MetricCard.svelte
      OperatingHours.svelte
      BayAssignment.svelte
      PromoBuilder.svelte
      StatsChart.svelte
      EmptyState.svelte
      NotificationBell.svelte
      VehicleTypeIcon.svelte

  styles/
    app.css                    # Tailwind + CSS variables (theme)
    typography.css             # Font imports, text utilities

  utils/
    format.ts                  # formatVND(), formatDate(), formatDuration()
    validation.ts              # Valibot schemas
```

## 12. Forms & Validation

Use `sveltekit-superforms` + `valibot` for all forms:

- Owner forms: location setup, service CRUD, promotion creation, profile
- Driver forms: booking, review, phone verification
- Inline validation with Vietnamese error messages
- Submit button shows loading state, disables on submit
- Success: toast notification via Sonner
- Error: inline field errors + toast for server errors

## 13. Charts & Data Visualization

Use `layerchart` (Svelte-native, built on D3):

- Revenue trend: line chart (daily/weekly)
- Jobs by hour: bar chart (heatmap-style for utilization)
- Top services: horizontal bar chart
- Queue wait time: real-time gauge or number
- Keep charts simple - no 3D, no excessive decoration
- VND formatting on Y-axis, Vietnamese date labels on X-axis
