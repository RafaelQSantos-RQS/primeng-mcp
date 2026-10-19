---
title: "Angular Toolbar Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/toolbar.md"
description: "Toolbar is a grouping component for buttons and other content."
---

# Angular Toolbar Component

Toolbar is a grouping component for buttons and other content.

## Accessibility

Screen Reader Toolbar uses toolbar role for the root element, aria-orientation is not included as it defaults to horizontal . Any valid attribute is passed to the root element so you may add additional properties like aria-labelledby and aria-labelled to define the element if required. Keyboard Support Component does not include any interactive elements. Arbitrary content can be placed with templating and elements like buttons inside should follow the page tab sequence.

## Basic

Combines action buttons and controls in a horizontal bar.

```typescript
import { Component } from '@angular/core';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { ToolbarModule } from 'primeng/toolbar';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { Plus } from '@primeicons/angular/plus';
import { Upload } from '@primeicons/angular/upload';

@Component({
    template: `
        <p-toolbar>
            <ng-template #start>
                <button pButton iconOnly class="mr-2" variant="text" severity="secondary"><svg data-p-icon="plus" /></button>
                <button pButton iconOnly class="mr-2" variant="text" severity="secondary"><i class="pi pi-print"></i></button>
                <button pButton iconOnly variant="text" severity="secondary"><svg data-p-icon="upload" /></button>
            </ng-template>
            <ng-template #center>
                <p-iconfield iconPosition="left">
                    <p-inputicon class="pi pi-search" />
                    <input type="text" pInputText placeholder="Search" />
                </p-iconfield>
            </ng-template>
            <ng-template #end>
                <button pButton severity="secondary" variant="outlined" size="small">Cancel</button>
                <button pButton size="small" class="ml-2">Save</button>
            </ng-template>
        </p-toolbar>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, ToolbarModule, ButtonModule, InputTextModule, Plus, Upload]
})
export class ToolbarBasicDemo {}
```

## Custom

A customized toolbar with navigation bar functionality.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { ToolbarModule } from 'primeng/toolbar';
import { ButtonModule } from 'primeng/button';
import { Search } from '@primeicons/angular/search';

@Component({
    template: `
        <p-toolbar [pt]="{ start: { class: 'flex-wrap' } }" [style]="{ 'border-radius': '3rem', padding: '0.75rem 1rem 0.75rem 1.5rem', background: 'var(--p-surface-900)', border: 'none' }">
            <ng-template #start>
                <div class="flex items-center gap-1">
                    <i class="pi pi-prime text-xl" style="color: var(--p-primary-400)"></i>
                    <span class="font-bold text-lg ml-2" style="color: var(--p-surface-0)">Brand</span>
                </div>
                <div class="ml-6 flex flex-wrap gap-1">
                    <button pButton variant="text" size="small" class="text-surface-0! hover:bg-surface-700!">Products</button>
                    <button pButton variant="text" size="small" class="text-surface-0! hover:bg-surface-700!">Solutions</button>
                    <button pButton variant="text" size="small" class="text-surface-0! hover:bg-surface-700!">Resources</button>
                    <button pButton variant="text" size="small" class="text-surface-0! hover:bg-surface-700!">Pricing</button>
                </div>
            </ng-template>
            <ng-template #end>
                <div class="flex items-center gap-2">
                    <button pButton variant="text" size="small" iconOnly aria-label="Search" class="text-surface-0! hover:bg-surface-700!"><svg data-p-icon="search" /></button>
                    <button pButton severity="contrast" size="small">Get Started</button>
                    <p-avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
                </div>
            </ng-template>
        </p-toolbar>
    `,
    standalone: true,
    imports: [AvatarModule, ToolbarModule, ButtonModule, Search]
})
export class ToolbarCustomDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { ToolbarModule } from 'primeng/toolbar';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { Plus } from '@primeicons/angular/plus';
import { Upload } from '@primeicons/angular/upload';

@Component({
    template: `
        <p-toolbar>
            <ng-template #start>
                <button pButton iconOnly class="mr-2" variant="text" severity="secondary"><svg data-p-icon="plus" /></button>
                <button pButton iconOnly class="mr-2" variant="text" severity="secondary"><i class="pi pi-print"></i></button>
                <button pButton iconOnly variant="text" severity="secondary"><svg data-p-icon="upload" /></button>
            </ng-template>
            <ng-template #center>
                <p-iconfield iconPosition="left">
                    <p-inputicon class="pi pi-search" />
                    <input type="text" pInputText placeholder="Search" />
                </p-iconfield>
            </ng-template>
            <ng-template #end>
                <button pButton severity="secondary" variant="outlined" size="small">Cancel</button>
                <button pButton size="small" class="ml-2">Save</button>
            </ng-template>
        </p-toolbar>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, ToolbarModule, ButtonModule, InputTextModule, Plus, Upload]
})
export class ToolbarPreviewDemo {}
```

## Toolbar

Toolbar is a grouping component for buttons and other content.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ToolbarPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| ariaLabelledBy | string \| undefined | - | Defines a string value that labels an interactive element. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| start | TemplateRef<void> \| undefined | Custom start template. |
| end | TemplateRef<void> \| undefined | Custom end template. |
| center | TemplateRef<void> \| undefined | Custom center template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| start | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the start's DOM element. |
| center | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the center's DOM element. |
| end | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the right's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-toolbar | Class name of the root element |
| p-toolbar-start | Class name of the start element |
| p-toolbar-center | Class name of the center element |
| p-toolbar-end | Class name of the end element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| toolbar.background | --p-toolbar-background | Background of root |
| toolbar.border.color | --p-toolbar-border-color | Border color of root |
| toolbar.border.radius | --p-toolbar-border-radius | Border radius of root |
| toolbar.color | --p-toolbar-color | Color of root |
| toolbar.gap | --p-toolbar-gap | Gap of root |
| toolbar.padding | --p-toolbar-padding | Padding of root |
