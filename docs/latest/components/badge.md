---
title: "Angular Badge Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/badge.md"
description: "Badge is a small status indicator for another element."
---

# Angular Badge Component

Badge is a small status indicator for another element.

## Accessibility

Screen Reader Badge does not include any roles and attributes by default, any attribute is passed to the root element so aria roles and attributes can be added if required. If the badges are dynamic, aria-live may be utilized as well. In case badges need to be tabbable, tabIndex can be added to implement custom key handlers. Keyboard Support Component does not include any interactive elements.

## Basic

A small overlay on another element to indicate a count or status.

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';

@Component({
    template: `
        <div class="flex justify-center">
            <p-badge value="Badge" />
        </div>
    `,
    standalone: true,
    imports: [BadgeModule]
})
export class BadgeBasicDemo {}
```

## Button

Buttons have built-in support for badges to display a badge inline.

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';
import { OverlayBadgeModule } from 'primeng/overlaybadge';
import { ButtonModule } from 'primeng/button';
import { Users } from '@primeicons/angular/users';
import { Bell } from '@primeicons/angular/bell';

@Component({
    template: `
        <div class="flex justify-center flex-wrap gap-4">
            <button type="button" pButton>Emails<p-badge value="8" severity="secondary" /></button>
            <button type="button" pButton variant="outlined"><svg data-p-icon="users"></svg>Messages<p-badge value="2" severity="contrast" /></button>
            <p-overlay-badge severity="info" styleClass="animate-pulse">
                <button type="button" pButton variant="outlined" iconOnly><svg data-p-icon="bell"></svg></button>
            </p-overlay-badge>
        </div>
    `,
    standalone: true,
    imports: [BadgeModule, OverlayBadgeModule, ButtonModule, Users, Bell]
})
export class BadgeButtonDemo {}
```

## Overlay

A badge can be added to any element by encapsulating the content with the OverlayBadge component.

```typescript
import { Component } from '@angular/core';
import { OverlayBadgeModule } from 'primeng/overlaybadge';
import { Bell } from '@primeicons/angular/bell';
import { Calendar } from '@primeicons/angular/calendar';
import { Envelope } from '@primeicons/angular/envelope';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-6">
            <p-overlaybadge value="2">
                <svg data-p-icon="bell" [size]="24"></svg>
            </p-overlaybadge>
            <p-overlaybadge value="4" severity="danger">
                <svg data-p-icon="calendar" [size]="24"></svg>
            </p-overlaybadge>
            <p-overlaybadge severity="danger">
                <svg data-p-icon="envelope" [size]="24"></svg>
            </p-overlaybadge>
        </div>
    `,
    standalone: true,
    imports: [OverlayBadgeModule, Bell, Calendar, Envelope]
})
export class BadgeOverlayDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';

@Component({
    template: `
        <div class="flex flex-wrap items-center justify-center gap-2 max-w-2xs mx-auto">
            <p-badge severity="success" value="Approved" />
            <p-badge severity="danger" value="Rejected" />
            <p-badge severity="warn" value="Warning" />
            <p-badge severity="info" value="Pending" />
            <p-badge severity="secondary" value="Draft" />
            <p-badge value="Featured" />
            <p-badge severity="info" value="3 New" />
            <p-badge severity="success" value="Online" />
            <p-badge severity="secondary" value="Private" />
            <p-badge severity="danger" value="Security issue" />
        </div>
    `,
    standalone: true,
    imports: [BadgeModule]
})
export class BadgePreviewDemo {}
```

## Severity

The severity property defines the visual style of a badge.

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2">
            <p-badge value="Default" />
            <p-badge value="Secondary" severity="secondary" />
            <p-badge value="Success" severity="success" />
            <p-badge value="Info" severity="info" />
            <p-badge value="Warning" severity="warn" />
            <p-badge value="Danger" severity="danger" />
            <p-badge value="Contrast" severity="contrast" />
        </div>
    `,
    standalone: true,
    imports: [BadgeModule]
})
export class BadgeSeverityDemo {}
```

## Size

Use the badgeSize property to change the size of a badge.

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';

@Component({
    template: `
        <div class="flex flex-wrap items-center justify-center gap-2">
            <p-badge value="Small" badgeSize="small" />
            <p-badge value="Default" />
            <p-badge value="Large" badgeSize="large" />
            <p-badge value="XLarge" badgeSize="xlarge" />
        </div>
    `,
    standalone: true,
    imports: [BadgeModule]
})
export class BadgeSizeDemo {}
```

## Badge

Badge is a small status indicator for another element.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| badgeSize | BadgeSize \| undefined | - | Size of the badge, valid options are "large" and "xlarge". |
| size | BadgeSize \| undefined | - | Size of the badge, valid options are "large" and "xlarge". |
| severity | BadgeSeverity \| undefined | - | Severity type of the badge. |
| value | string \| number \| null \| undefined | - | Value to display inside the badge. |
| badgeDisabled | boolean | - | When specified, disables the component. |
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, BadgePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-badge | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| badge.border.radius | --p-badge-border-radius | Border radius of root |
| badge.padding | --p-badge-padding | Padding of root |
| badge.font.size | --p-badge-font-size | Font size of root |
| badge.font.weight | --p-badge-font-weight | Font weight of root |
| badge.min.width | --p-badge-min-width | Min width of root |
| badge.height | --p-badge-height | Height of root |
| badge.dot.size | --p-badge-dot-size | Size of dot |
| badge.sm.font.size | --p-badge-sm-font-size | Font size of sm |
| badge.sm.min.width | --p-badge-sm-min-width | Min width of sm |
| badge.sm.height | --p-badge-sm-height | Height of sm |
| badge.lg.font.size | --p-badge-lg-font-size | Font size of lg |
| badge.lg.min.width | --p-badge-lg-min-width | Min width of lg |
| badge.lg.height | --p-badge-lg-height | Height of lg |
| badge.xl.font.size | --p-badge-xl-font-size | Font size of xl |
| badge.xl.min.width | --p-badge-xl-min-width | Min width of xl |
| badge.xl.height | --p-badge-xl-height | Height of xl |
| badge.primary.background | --p-badge-primary-background | Background of primary |
| badge.primary.color | --p-badge-primary-color | Color of primary |
| badge.secondary.background | --p-badge-secondary-background | Background of secondary |
| badge.secondary.color | --p-badge-secondary-color | Color of secondary |
| badge.success.background | --p-badge-success-background | Background of success |
| badge.success.color | --p-badge-success-color | Color of success |
| badge.info.background | --p-badge-info-background | Background of info |
| badge.info.color | --p-badge-info-color | Color of info |
| badge.warn.background | --p-badge-warn-background | Background of warn |
| badge.warn.color | --p-badge-warn-color | Color of warn |
| badge.danger.background | --p-badge-danger-background | Background of danger |
| badge.danger.color | --p-badge-danger-color | Color of danger |
| badge.contrast.background | --p-badge-contrast-background | Background of contrast |
| badge.contrast.color | --p-badge-contrast-color | Color of contrast |
