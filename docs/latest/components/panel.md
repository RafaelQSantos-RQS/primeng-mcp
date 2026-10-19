---
title: "Angular Panel Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/panel.md"
description: "Panel is a container component with an optional content toggle feature."
---

# Angular Panel Component

Panel is a container component with an optional content toggle feature.

## Accessibility

Screen Reader Toggleable panels use a content toggle button at the header that has aria-controls to define the id of the content section along with aria-expanded for the visibility state. The value to read the button defaults to the value of the header property and can be customized by defining an aria-label or aria-labelledby via the toggleButtonProps property. The content uses region , defines an id that matches the aria-controls of the content toggle button and aria-labelledby referring to the id of the header. Content Toggle Button Keyboard Support Key Function tab Moves focus to the next the focusable element in the page tab sequence. shift + tab Moves focus to the previous the focusable element in the page tab sequence. enter Toggles the visibility of the content. space Toggles the visibility of the content.

## Basic

A simple Panel is created with a header property along with the content as children.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { PanelModule } from 'primeng/panel';

@Component({
    template: `
        <div class="flex justify-center">
            <p-panel header="Order Summary" class="max-w-xs w-full">
                <div class="flex flex-col text-sm">
                    <div class="flex justify-between">
                        <span class="text-muted-color">Wireless Headphones</span>
                        <span class="text-color font-medium">$79.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Phone Case</span>
                        <span class="text-color font-medium">$15.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Shipping</span>
                        <span class="text-color font-medium">$5.99</span>
                    </div>
                    <p-divider />
                    <div class="flex justify-between">
                        <span class="text-color font-semibold">Total</span>
                        <span class="text-color font-semibold">$99.99</span>
                    </div>
                </div>
            </p-panel>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, PanelModule]
})
export class PanelBasicDemo {}
```

## Controlled

The collapsed property with two-way binding controls the open state programmatically.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { PanelModule } from 'primeng/panel';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="space-y-4 max-w-xs w-full">
                <div class="flex gap-2 justify-center">
                    <button type="button" pButton (click)="collapsed = false" [severity]="!collapsed ? 'primary' : 'secondary'">Open</button>
                    <button type="button" pButton (click)="collapsed = true" [severity]="collapsed ? 'primary' : 'secondary'">Close</button>
                </div>
                <p-panel header="Order Summary" [toggleable]="true" [(collapsed)]="collapsed">
                    <div class="flex flex-col text-sm">
                        <div class="flex justify-between">
                            <span class="text-muted-color">Wireless Headphones</span>
                            <span class="text-color font-medium">$79.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Phone Case</span>
                            <span class="text-color font-medium">$15.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Shipping</span>
                            <span class="text-color font-medium">$5.99</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-semibold">Total</span>
                            <span class="text-color font-semibold">$99.99</span>
                        </div>
                    </div>
                </p-panel>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, PanelModule, ButtonModule]
})
export class PanelControlledDemo {
    collapsed: boolean = false;
}
```

## Indicator

The headericons template customizes the indicator. It can switch between two icons using the collapsed context variable, or render a single icon that animates purely with CSS by targeting the toggle button's aria-expanded state.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { PanelModule } from 'primeng/panel';
import { Plus } from '@primeicons/angular/plus';
import { Minus } from '@primeicons/angular/minus';
import { ChevronDown } from '@primeicons/angular/chevron-down';

@Component({
    template: `
        <div class="flex flex-col gap-8 max-w-xs mx-auto">
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">Match open / closed</h3>
                <p-panel header="Order Summary" [toggleable]="true" class="w-full">
                    <ng-template #headericons let-collapsed="collapsed">
                        @if (collapsed) {
                            <svg data-p-icon="plus"></svg>
                        } @else {
                            <svg data-p-icon="minus"></svg>
                        }
                    </ng-template>
                    <div class="flex flex-col text-sm">
                        <div class="flex justify-between">
                            <span class="text-muted-color">Wireless Headphones</span>
                            <span class="text-color font-medium">$79.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Phone Case</span>
                            <span class="text-color font-medium">$15.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Shipping</span>
                            <span class="text-color font-medium">$5.99</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-semibold">Total</span>
                            <span class="text-color font-semibold">$99.99</span>
                        </div>
                    </div>
                </p-panel>
            </div>
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">CSS-only with data attributes</h3>
                <p-panel header="Order Summary" [toggleable]="true" class="w-full">
                    <ng-template #headericons>
                        <svg data-p-icon="chevron-down" class="transition-transform duration-200 in-aria-expanded:rotate-180"></svg>
                    </ng-template>
                    <div class="flex flex-col text-sm">
                        <div class="flex justify-between">
                            <span class="text-muted-color">Wireless Headphones</span>
                            <span class="text-color font-medium">$79.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Phone Case</span>
                            <span class="text-color font-medium">$15.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-muted-color">Shipping</span>
                            <span class="text-color font-medium">$5.99</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-semibold">Total</span>
                            <span class="text-color font-semibold">$99.99</span>
                        </div>
                    </div>
                </p-panel>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, PanelModule, Plus, Minus, ChevronDown]
})
export class PanelIndicatorDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { PanelModule } from 'primeng/panel';

@Component({
    template: `
        <div class="flex justify-center">
            <p-panel header="Order Summary" [toggleable]="true" class="max-w-xs w-full">
                <div class="flex flex-col text-sm">
                    <div class="flex justify-between">
                        <span class="text-muted-color">Wireless Headphones</span>
                        <span class="text-color font-medium">$79.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Phone Case</span>
                        <span class="text-color font-medium">$15.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Shipping</span>
                        <span class="text-color font-medium">$5.99</span>
                    </div>
                    <p-divider />
                    <div class="flex justify-between">
                        <span class="text-color font-semibold">Total</span>
                        <span class="text-color font-semibold">$99.99</span>
                    </div>
                </div>
            </p-panel>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, PanelModule]
})
export class PanelPreviewDemo {}
```

## Template

Header and footer sections can be customized using header and footer templates.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { PanelModule } from 'primeng/panel';
import { ButtonModule } from 'primeng/button';
import { Product } from '@/domain/product';
import { Cog } from '@primeicons/angular/cog';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { User } from '@primeicons/angular/user';
import { Bookmark } from '@primeicons/angular/bookmark';

@Component({
    template: `
        <div class="flex justify-center">
            <p-panel [toggleable]="true" class="max-w-xs w-full" [pt]="{ header: 'p-4!' }">
                <ng-template #header>
                    <div class="flex items-center gap-2">
                        <p-avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
                        <span class="font-bold">Amy Elsner</span>
                    </div>
                </ng-template>
                <ng-template #icons>
                    <button type="button" pButton iconOnly severity="secondary" rounded text>
                        <svg data-p-icon="cog"></svg>
                    </button>
                </ng-template>
                <ng-template #headericons let-collapsed>
                    <svg data-p-icon="chevron-down" [class]="collapsed ? 'transition-transform duration-200' : 'transition-transform duration-200 rotate-180'"></svg>
                </ng-template>
                <ng-template #footer>
                    <div class="flex flex-wrap items-center justify-between gap-4">
                        <div class="flex items-center gap-2">
                            <button type="button" pButton iconOnly rounded text>
                                <svg data-p-icon="user"></svg>
                            </button>
                            <button type="button" pButton iconOnly severity="secondary" rounded text>
                                <svg data-p-icon="bookmark"></svg>
                            </button>
                        </div>
                        <span class="text-surface-500 dark:text-surface-400 text-sm">Updated 2 hours ago</span>
                    </div>
                </ng-template>
                <p class="m-0 text-sm">
                    Product designer focused on accessible interfaces, scalable design systems and modern frontend workflows. Passionate about creating intuitive user experiences that balance usability, consistency and performance across
                    platforms. Currently exploring AI-assisted UI tooling, component architecture and developer experience improvements to streamline design-to-development collaboration and build more maintainable products.
                </p>
            </p-panel>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, PanelModule, ButtonModule, Cog, ChevronDown, User, Bookmark]
})
export class PanelTemplateDemo {}
```

## Toggleable

Content of the panel can be expanded and collapsed using the toggleable property.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { PanelModule } from 'primeng/panel';

@Component({
    template: `
        <div class="flex justify-center">
            <p-panel header="Order Summary" [toggleable]="true" class="max-w-xs w-full">
                <div class="flex flex-col text-sm">
                    <div class="flex justify-between">
                        <span class="text-muted-color">Wireless Headphones</span>
                        <span class="text-color font-medium">$79.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Phone Case</span>
                        <span class="text-color font-medium">$15.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-muted-color">Shipping</span>
                        <span class="text-color font-medium">$5.99</span>
                    </div>
                    <p-divider />
                    <div class="flex justify-between">
                        <span class="text-color font-semibold">Total</span>
                        <span class="text-color font-semibold">$99.99</span>
                    </div>
                </div>
            </p-panel>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, PanelModule]
})
export class PanelToggleableDemo {}
```

## Panel

Panel is a container with the optional content toggle feature.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, PanelPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| toggleable | boolean | - | Defines if content of panel can be expanded and collapsed. |
| header | string \| undefined | - | Header text of the panel. |
| collapsed | boolean | - | Defines the initial state of panel content, supports one or two-way binding as well. |
| iconPos | "start" \| "end" \| "center" | - | Position of the icons. |
| showHeader | boolean | - | Specifies if header of panel cannot be displayed. |
| toggler | "icon" \| "header" | - | Specifies the toggler element to toggle the panel content. |
| toggleButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the toggle button. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onBeforeToggle | event: PanelBeforeToggleEvent | Callback to invoke before panel toggle. |
| onAfterToggle | event: PanelAfterToggleEvent | Callback to invoke after panel toggle. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<void> \| undefined | Defines template option for header. |
| icon | TemplateRef<void> \| undefined | Defines template option for icon. |
| content | TemplateRef<void> \| undefined | Defines template option for content. |
| footer | TemplateRef<void> \| undefined | Defines template option for footer. |
| headericons | TemplateRef<PanelHeaderIconsTemplateContext> \| undefined | Defines template option for headerIcon. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| title | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the title's DOM element. |
| headerActions | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header actions' DOM element. |
| pcToggleButton | ButtonPassThrough | Used to pass attributes to the toggle button button's DOM element. |
| contentContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content container's DOM element. |
| contentWrapper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content wrapper DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-panel | Class name of the root element |
| p-panel-header | Class name of the header element |
| p-panel-title | Class name of the title element |
| p-panel-header-actions | Class name of the header actions element |
| p-panel-toggle-button | Class name of the toggle button element |
| p-panel-content-container | Class name of the content container element |
| p-panel-content-wrapper | Class name of the content wrapper element |
| p-panel-content | Class name of the content element |
| p-panel-footer | Class name of the footer element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| panel.background | --p-panel-background | Background of root |
| panel.border.color | --p-panel-border-color | Border color of root |
| panel.color | --p-panel-color | Color of root |
| panel.border.radius | --p-panel-border-radius | Border radius of root |
| panel.header.background | --p-panel-header-background | Background of header |
| panel.header.color | --p-panel-header-color | Color of header |
| panel.header.padding | --p-panel-header-padding | Padding of header |
| panel.header.border.color | --p-panel-header-border-color | Border color of header |
| panel.header.border.width | --p-panel-header-border-width | Border width of header |
| panel.header.border.radius | --p-panel-header-border-radius | Border radius of header |
| panel.toggleable.header.padding | --p-panel-toggleable-header-padding | Padding of toggleable header |
| panel.title.font.weight | --p-panel-title-font-weight | Font weight of title |
| panel.title.font.size | --p-panel-title-font-size | Font size of title |
| panel.content.padding | --p-panel-content-padding | Padding of content |
| panel.footer.padding | --p-panel-footer-padding | Padding of footer |
