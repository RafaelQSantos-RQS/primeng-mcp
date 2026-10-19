---
title: "Angular Tag Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/tag.md"
description: "Tag component is used to categorize content."
---

# Angular Tag Component

Tag component is used to categorize content.

## Accessibility

Screen Reader Tag does not include any roles and attributes by default, any attribute is passed to the root element so aria roles and attributes can be added if required. If the tags are dynamic, aria-live may be utilized as well. In case badges need to be tabbable, tabIndex can be added to implement custom key handlers. Keyboard Support Component does not include any interactive elements.

## Basic

A small label for categorizing or marking items.

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <p-tag severity="contrast" value="Highlighted" />
        </div>
    `,
    standalone: true,
    imports: [TagModule]
})
export class TagBasicDemo {}
```

## Icon

Place the icon left or right of the label.

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { User } from '@primeicons/angular/user';
import { Check } from '@primeicons/angular/check';
import { Search } from '@primeicons/angular/search';
import { ExclamationTriangle } from '@primeicons/angular/exclamation-triangle';
import { Times } from '@primeicons/angular/times';
import { Cog } from '@primeicons/angular/cog';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2">
            <p-tag>
                <svg data-p-icon="user" />
                Primary
            </p-tag>
            <p-tag severity="secondary">
                <svg data-p-icon="user" />
                Secondary
            </p-tag>
            <p-tag severity="success">
                <svg data-p-icon="check" />
                Success
            </p-tag>
            <p-tag severity="info">
                <svg data-p-icon="search" />
                Info
            </p-tag>
            <p-tag severity="warn">
                <svg data-p-icon="exclamation-triangle" />
                Warn
            </p-tag>
            <p-tag severity="danger">
                <svg data-p-icon="times" />
                Danger
            </p-tag>
            <p-tag severity="contrast">
                <svg data-p-icon="cog" />
                Contrast
            </p-tag>
        </div>
    `,
    standalone: true,
    imports: [TagModule, User, Check, Search, ExclamationTriangle, Times, Cog]
})
export class TagIconDemo {}
```

## Pill

Enabling rounded , displays a tag as a pill.

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2">
            <p-tag value="Primary" [rounded]="true" />
            <p-tag severity="secondary" value="Secondary" [rounded]="true" />
            <p-tag severity="success" value="Success" [rounded]="true" />
            <p-tag severity="info" value="Info" [rounded]="true" />
            <p-tag severity="warn" value="Warn" [rounded]="true" />
            <p-tag severity="danger" value="Danger" [rounded]="true" />
            <p-tag severity="contrast" value="Contrast" [rounded]="true" />
        </div>
    `,
    standalone: true,
    imports: [TagModule]
})
export class TagPillDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { InfoCircle } from '@primeicons/angular/info-circle';
import { ExclamationTriangle } from '@primeicons/angular/exclamation-triangle';
import { TimesCircle } from '@primeicons/angular/times-circle';
import { Clock } from '@primeicons/angular/clock';
import { Check } from '@primeicons/angular/check';

@Component({
    template: `
        <div class="flex items-center justify-center flex-wrap gap-3 max-w-xs mx-auto">
            <p-tag severity="secondary" value="Draft" />
            <p-tag severity="info">
                <svg data-p-icon="info-circle" />
                Info
            </p-tag>
            <p-tag severity="success" [rounded]="true" value="Active" />
            <p-tag severity="warn">
                <svg data-p-icon="exclamation-triangle" />
                Attention
            </p-tag>
            <p-tag severity="danger" [rounded]="true">
                <svg data-p-icon="times-circle" />
                Error
            </p-tag>
            <p-tag severity="contrast" value="Featured" />
            <p-tag severity="info" value="New" />
            <p-tag severity="secondary" [rounded]="true" value="Archived" />
            <p-tag severity="warn" [rounded]="true">
                <svg data-p-icon="clock" />
                Pending
            </p-tag>
            <p-tag severity="success">
                <svg data-p-icon="check" />
                Verified
            </p-tag>
        </div>
    `,
    standalone: true,
    imports: [TagModule, InfoCircle, ExclamationTriangle, TimesCircle, Clock, Check]
})
export class TagPreviewDemo {}
```

## Severity

Severity defines the color of the tag, possible values are success , info , warn and danger in addition to the default theme color.

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2">
            <p-tag value="Primary" />
            <p-tag severity="secondary" value="Secondary" />
            <p-tag severity="success" value="Success" />
            <p-tag severity="info" value="Info" />
            <p-tag severity="warn" value="Warn" />
            <p-tag severity="danger" value="Danger" />
            <p-tag severity="contrast" value="Contrast" />
        </div>
    `,
    standalone: true,
    imports: [TagModule]
})
export class TagSeverityDemo {}
```

## Template

Children of the component are passed as the content for templating.

```typescript
import { Component } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { Country } from '@/domain/customer';

@Component({
    template: `
        <div class="flex justify-center">
            <p-tag class="flex items-center gap-2 px-3" [style]="{ border: '2px solid var(--border-color)', background: 'transparent', color: 'var(--text-color)' }">
                <img alt="Country" src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" class="flag flag-it" style="width: 18px" />
                <span class="text-base">Italy</span>
            </p-tag>
        </div>
    `,
    standalone: true,
    imports: [TagModule]
})
export class TagTemplateDemo {}
```

## Tag

Tag component is used to categorize content.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, TagPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| severity | TagSeverity \| null \| undefined | - | Severity type of the tag. |
| value | string \| undefined | - | Value to display inside the tag. |
| icon | string \| undefined | - | Icon of the tag to display next to the value. |
| rounded | boolean | - | Whether the corners of the tag are rounded. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| icon | TemplateRef<void> \| undefined | Custom icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the root's DOM element. |
| icon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the icon's DOM element. |
| label | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the label's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-tag | Class name of the root element |
| p-tag-icon | Class name of the icon element |
| p-tag-label | Class name of the label element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| tag.font.size | --p-tag-font-size | Font size of root |
| tag.font.weight | --p-tag-font-weight | Font weight of root |
| tag.padding | --p-tag-padding | Padding of root |
| tag.gap | --p-tag-gap | Gap of root |
| tag.border.radius | --p-tag-border-radius | Border radius of root |
| tag.rounded.border.radius | --p-tag-rounded-border-radius | Rounded border radius of root |
| tag.icon.size | --p-tag-icon-size | Size of icon |
| tag.primary.background | --p-tag-primary-background | Background of primary |
| tag.primary.color | --p-tag-primary-color | Color of primary |
| tag.secondary.background | --p-tag-secondary-background | Background of secondary |
| tag.secondary.color | --p-tag-secondary-color | Color of secondary |
| tag.success.background | --p-tag-success-background | Background of success |
| tag.success.color | --p-tag-success-color | Color of success |
| tag.info.background | --p-tag-info-background | Background of info |
| tag.info.color | --p-tag-info-color | Color of info |
| tag.warn.background | --p-tag-warn-background | Background of warn |
| tag.warn.color | --p-tag-warn-color | Color of warn |
| tag.danger.background | --p-tag-danger-background | Background of danger |
| tag.danger.color | --p-tag-danger-color | Color of danger |
| tag.contrast.background | --p-tag-contrast-background | Background of contrast |
| tag.contrast.color | --p-tag-contrast-color | Color of contrast |
