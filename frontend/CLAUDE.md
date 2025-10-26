# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**EduTrack Pro** is a continuing education management platform for professionals (lawyers, engineers, CPAs, architects) to track their CE requirements, get AI-powered course recommendations, and automate certification submissions to governing bodies.

This is a hackathon project built with SvelteKit 2 and Svelte Material UI (SMUI).

## Technology Stack

- **Framework**: SvelteKit 2 with Svelte 5 (using new runes syntax: `$state`, `$effect`, `$props`)
- **UI Components**: Svelte Material UI (SMUI) v8
- **Styling**: SCSS with Material Design theming
- **TypeScript**: Enabled for type safety
- **Build Tool**: Vite

## Development Commands

```bash
# Install dependencies
npm install

# Start development server (runs on http://localhost:5173)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking
npm run check

# Type checking in watch mode
npm run check:watch
```

## Project Structure

```
src/
├── routes/
│   ├── +page.svelte           # Login page (/)
│   ├── +layout.svelte          # Root layout (imports theme and fonts)
│   ├── onboarding/
│   │   └── +page.svelte        # Onboarding page (/onboarding)
│   └── dashboard/
│       └── +page.svelte        # Dashboard page (/dashboard)
├── theme.scss                  # Global SMUI theme configuration
└── lib/                        # Shared utilities (currently minimal)
```

## Design System

### Color Theme
- **Primary**: `#3B82F6` (Bright Blue)
- **Secondary**: `#A855F7` (Vibrant Purple)
- **Background**: `#fafafa` (Light gray for dashboard)
- **Gradients**: Blue-to-purple gradients used throughout for modern AI/tech aesthetic

### Typography
- **Font**: Inter (loaded from Google Fonts)
- **Material Icons**: Used for all icons

### Component Library
Uses SMUI (Svelte Material UI) components:
- `Button` - Action buttons with variants (raised, outlined)
- `Card` - Container components for content sections
- `Textfield` - Form input fields with Material Design styling

## Page Architecture

### 1. Login Page (`/`)
- Simple authentication form (mock - no backend yet)
- Semantic HTML (`<main>`, `<section>`, `<header>`, `<form>`)
- Gradient background with centered card
- Navigates to `/onboarding` on submit
- Accessible with ARIA labels and 44px touch targets

### 2. Onboarding Page (`/onboarding`)
**Key Features:**
- Multi-section form with step indicators
- **Auto-population logic**: CE hours required are auto-filled based on profession + state selection
- **AI topic suggestions**: Displays profession-specific topics as interactive chips
- Custom topic input with "Add" button
- Progress preview showing CE completion status
- Mobile-responsive with adaptive layouts

**Data Structures:**
- `ceRequirements`: Maps profession → state → hours (with fallback defaults)
- `topicSuggestions`: Maps profession → array of relevant topics
- Uses Svelte 5 `$effect` rune to reactively update CE hours and topics

**Navigation**: Goes to `/dashboard` on submit

### 3. Dashboard Page (`/dashboard`)
**Key Features:**
- Top navigation bar with app branding and nav items
- Welcome section with user greeting
- **Stats grid**:
  - CE Hours progress card with circular progress indicator
  - Next renewal deadline card
  - Completion status card
- **AI-powered recommendations**: Course cards with:
  - AI reasoning for why course is recommended
  - Course metadata (hours, format, price, rating)
  - Topic badges matching user interests
- **Recent activity**: List of completed courses with certification status

**Mock Data**: Currently uses hardcoded mock data for demonstration
- User profile (name, profession, state, license info)
- CE hours (18/25 completed)
- Selected topics from onboarding
- Course recommendations with ratings and metadata
- Recent completed courses

**Visual Features:**
- SVG circular progress indicator with gradient stroke
- Hover effects on course cards
- Gradient buttons and badges
- Material icons throughout

## Accessibility

All pages follow accessibility best practices:
- Semantic HTML5 elements (`<main>`, `<section>`, `<header>`, `<nav>`)
- ARIA labels on interactive elements
- `aria-pressed` for toggle buttons (topic chips)
- `required` attributes on form fields
- Visible focus indicators (2-3px outlines)
- Minimum 44px touch targets for mobile
- Color contrast meets WCAG AA standards
- Progress bars have proper `role="progressbar"` with aria values

## Responsive Design

Mobile-first responsive breakpoints:
- **Desktop**: Default (1024px+)
- **Tablet**: 768px - 1024px
  - Two-column layouts become single column
  - Navigation labels hidden, icons only
- **Mobile**: < 768px
  - Single column layouts
  - Reduced font sizes
  - Stacked button groups
  - Reduced padding

## State Management

Currently using Svelte 5 local component state with `$state` rune:
- No global store yet (suitable for hackathon MVP)
- Data passed between pages via navigation (not persisted)
- For production: Would add stores in `src/lib/stores/` for user data, CE progress, etc.

## Svelte 5 Runes Used

This project uses Svelte 5's new runes syntax:
- `$state()` - Reactive local state
- `$props()` - Component props
- `$effect()` - Side effects that run when dependencies change
- `{@render children?.()}` - Render prop slots

## Styling Conventions

1. **Scoped styles**: All styles are scoped to components
2. **Global SMUI classes**: Use `:global()` wrapper for SMUI overrides
3. **Responsive**: Always include mobile media queries
4. **Gradients**: Use blue-to-purple gradients for primary actions/highlights
5. **Spacing**: Consistent use of rem units (0.5rem, 1rem, 1.5rem, 2rem)

## Key Design Patterns

### Auto-population Pattern
```typescript
$effect(() => {
  if (profession && selectedState) {
    const hours = ceRequirements[profession]?.[selectedState] || ceRequirements[profession]?.['default'];
    hoursRequired = hours.toString();
  }
});
```

### Topic Selection Pattern
Interactive chips that toggle on/off with visual feedback:
```svelte
<button
  class:selected={selectedTopics.includes(topic)}
  aria-pressed={selectedTopics.includes(topic)}
  on:click={() => toggleTopic(topic)}
>
```

### Navigation Pattern
Using SvelteKit's `goto` for programmatic navigation:
```typescript
import { goto } from '$app/navigation';
goto('/dashboard');
```

## Future Enhancements (Post-Hackathon)

1. **Backend Integration**:
   - User authentication and session management
   - API for CE requirements by state/profession
   - Course catalog API integration
   - Certification submission to state bars

2. **AI Features**:
   - Real AI course recommendations (OpenAI/Anthropic API)
   - Natural language course search
   - Smart deadline reminders

3. **State Management**:
   - Add Svelte stores for persistent user data
   - Local storage for offline access

4. **Additional Pages**:
   - Course catalog with search/filter
   - Course detail pages
   - User profile settings
   - Certification submission history

## Development Notes

- This is a **demo/hackathon project** - authentication is mocked, data is hardcoded
- The AI features are simulated with hardcoded recommendations
- No backend or API calls currently implemented
- Focus is on UI/UX demonstration and rapid prototyping
