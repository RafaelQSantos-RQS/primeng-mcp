---
title: "Angular Fieldset Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/fieldset.md"
description: "Fieldset is a grouping component with a content toggle feature."
---

# Angular Fieldset Component

Fieldset is a grouping component with a content toggle feature.

## Accessibility

Screen Reader Fieldset component uses the semantic fieldset element. When toggleable option is enabled, a clickable element with button role is included inside the legend element, this button has aria-controls to define the id of the content section along with aria-expanded for the visibility state. The value to read the button defaults to the value of the legend property and can be customized by defining an aria-label or aria-labelledby via the toggleButtonProps property. The content uses region , defines an id that matches the aria-controls of the content toggle button and aria-labelledby referring to the id of the header. Content Toggle Button Keyboard Support Key Function tab Moves focus to the next the focusable element in the page tab sequence. shift + tab Moves focus to the previous the focusable element in the page tab sequence. enter Toggles the visibility of the content. space Toggles the visibility of the content.

## Basic

A simple Fieldset is created with a legend property along with the content as children.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { FieldsetModule } from 'primeng/fieldset';

@Component({
    template: `
        <div class="flex justify-center">
            <p-fieldset legend="Invoice #1024" class="max-w-xs w-full">
                <div class="flex flex-col text-sm p-2">
                    <div class="flex justify-between">
                        <span class="text-color">Design Service</span>
                        <span class="text-muted-color">$120.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-color">Hosting</span>
                        <span class="text-muted-color">$30.00</span>
                    </div>
                    <p-divider />
                    <div class="flex justify-between">
                        <span class="text-color font-medium">Total</span>
                        <span class="text-color font-medium">$150.00</span>
                    </div>
                </div>
            </p-fieldset>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, FieldsetModule]
})
export class FieldsetBasicDemo {}
```

## Controlled

The collapsed property with two-way binding controls the open state programmatically.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { FieldsetModule } from 'primeng/fieldset';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="space-y-4 max-w-xs w-full">
                <div class="flex gap-2 justify-center">
                    <button type="button" pButton (click)="collapsed = false" [severity]="!collapsed ? 'primary' : 'secondary'">Open</button>
                    <button type="button" pButton (click)="collapsed = true" [severity]="collapsed ? 'primary' : 'secondary'">Close</button>
                </div>
                <p-fieldset legend="Invoice #1024" [toggleable]="true" [(collapsed)]="collapsed">
                    <div class="flex flex-col text-sm p-2">
                        <div class="flex justify-between">
                            <span class="text-color">Design Service</span>
                            <span class="text-muted-color">$120.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-color">Hosting</span>
                            <span class="text-muted-color">$30.00</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-medium">Total</span>
                            <span class="text-color font-medium">$150.00</span>
                        </div>
                    </div>
                </p-fieldset>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, FieldsetModule, ButtonModule]
})
export class FieldsetControlledDemo {
    collapsed: boolean = false;
}
```

## Indicator

The toggleicon template customizes the indicator. It can switch between two icons using the collapsed context variable, or render a single icon that animates purely with CSS by targeting the toggle button's aria-expanded state.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { FieldsetModule } from 'primeng/fieldset';
import { Plus } from '@primeicons/angular/plus';
import { Minus } from '@primeicons/angular/minus';
import { ChevronDown } from '@primeicons/angular/chevron-down';

@Component({
    template: `
        <div class="flex flex-col gap-8 max-w-xs mx-auto">
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">Match open / closed</h3>
                <p-fieldset legend="Invoice #1024" [toggleable]="true" class="w-full">
                    <ng-template #toggleicon let-collapsed="collapsed">
                        @if (collapsed) {
                            <svg data-p-icon="plus" />
                        } @else {
                            <svg data-p-icon="minus" />
                        }
                    </ng-template>
                    <div class="flex flex-col text-sm p-2">
                        <div class="flex justify-between">
                            <span class="text-color">Design Service</span>
                            <span class="text-muted-color">$120.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-color">Hosting</span>
                            <span class="text-muted-color">$30.00</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-medium">Total</span>
                            <span class="text-color font-medium">$150.00</span>
                        </div>
                    </div>
                </p-fieldset>
            </div>
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">CSS-only with data attributes</h3>
                <p-fieldset legend="Invoice #1024" [toggleable]="true" class="w-full">
                    <ng-template #toggleicon>
                        <svg data-p-icon="chevron-down" class="transition-transform duration-200 in-aria-expanded:rotate-180" />
                    </ng-template>
                    <div class="flex flex-col text-sm p-2">
                        <div class="flex justify-between">
                            <span class="text-color">Design Service</span>
                            <span class="text-muted-color">$120.00</span>
                        </div>
                        <div class="flex justify-between mt-3">
                            <span class="text-color">Hosting</span>
                            <span class="text-muted-color">$30.00</span>
                        </div>
                        <p-divider />
                        <div class="flex justify-between">
                            <span class="text-color font-medium">Total</span>
                            <span class="text-color font-medium">$150.00</span>
                        </div>
                    </div>
                </p-fieldset>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, FieldsetModule, Plus, Minus, ChevronDown]
})
export class FieldsetIndicatorDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FieldsetModule } from 'primeng/fieldset';

@Component({
    template: `
        <div class="flex justify-center">
            <p-fieldset legend="Shipping Address" class="max-w-xs w-full">
                <div class="flex flex-col gap-6 text-sm p-2">
                    <div class="flex gap-8">
                        <div>
                            <div class="text-color font-medium">Name</div>
                            <div class="text-muted-color">Jane Cooper</div>
                        </div>
                        <div>
                            <div class="text-color font-medium">Phone</div>
                            <div class="text-muted-color">(415) 555-0132</div>
                        </div>
                    </div>
                    <div>
                        <div class="text-color font-medium">Address</div>
                        <div class="text-muted-color">1234 Elm Street, Apt 5B</div>
                        <div class="text-muted-color">San Francisco, CA 94102</div>
                    </div>
                </div>
            </p-fieldset>
        </div>
    `,
    standalone: true,
    imports: [FieldsetModule]
})
export class FieldsetPreviewDemo {}
```

## Template

Header section can also be defined with custom content instead of primitive values.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { FieldsetModule } from 'primeng/fieldset';

@Component({
    template: `
        <p-fieldset>
            <ng-template #header>
                <div class="flex items-center gap-2 px-2">
                    <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape="circle" />
                    <span class="font-bold">Amy Elsner</span>
                </div>
            </ng-template>
            <p class="m-0 text-sm">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </p>
        </p-fieldset>
    `,
    standalone: true,
    imports: [AvatarModule, FieldsetModule]
})
export class FieldsetTemplateDemo {}
```

## Toggleable

Content of the fieldset can be expanded and collapsed using the toggleable property.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';
import { FieldsetModule } from 'primeng/fieldset';

@Component({
    template: `
        <div class="flex justify-center">
            <p-fieldset legend="Invoice #1024" [toggleable]="true" class="max-w-xs w-full">
                <div class="flex flex-col text-sm p-2">
                    <div class="flex justify-between">
                        <span class="text-color">Design Service</span>
                        <span class="text-muted-color">$120.00</span>
                    </div>
                    <div class="flex justify-between mt-3">
                        <span class="text-color">Hosting</span>
                        <span class="text-muted-color">$30.00</span>
                    </div>
                    <p-divider />
                    <div class="flex justify-between">
                        <span class="text-color font-medium">Total</span>
                        <span class="text-color font-medium">$150.00</span>
                    </div>
                </div>
            </p-fieldset>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, FieldsetModule]
})
export class FieldsetToggleableDemo {}
```

## Fieldset

Fieldset is a grouping component with the optional content toggle feature.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, FieldsetPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| legend | string \| undefined | - | Header text of the fieldset. |
| toggleable | boolean | false | When specified, content can toggled by clicking the legend. |
| collapsed | boolean | - | Defines the initial state of content, supports one or two-way binding as well. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| styleClass | string \| undefined | - | Style class of the component. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onBeforeToggle | event: FieldsetBeforeToggleEvent | Callback to invoke before panel toggle. |
| onAfterToggle | event: FieldsetAfterToggleEvent | Callback to invoke after panel toggle. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<void> \| undefined | Custom header template. |
| toggleicon | TemplateRef<FieldsetToggleIconTemplateContext> \| undefined | Custom toggler icon template. |
| expandicon | TemplateRef<void> \| undefined | Custom expand icon template. |
| collapseicon | TemplateRef<void> \| undefined | Custom collapse icon template. |
| content | TemplateRef<void> \| undefined | Custom content template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLFieldSetElement, I> | Used to pass attributes to the root's DOM element. |
| legend | PassThroughOption<HTMLLegendElement, I> | Used to pass attributes to the legend's DOM element. |
| toggleButton | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the toggle button's DOM element. |
| toggleIcon | PassThroughOption<HTMLSpanElement \| SVGElement, I> | Used to pass attributes to the toggle icon's DOM element. |
| legendLabel | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the legend label's DOM element. |
| contentContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content container's DOM element. |
| contentWrapper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content wrapper DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-fieldset | Class name of the root element |
| p-fieldset-legend | Class name of the legend element |
| p-fieldset-legend-label | Class name of the legend label element |
| p-fieldset-toggle-icon | Class name of the toggle icon element |
| p-fieldset-content-container | Class name of the content container element |
| p-fieldset-content-wrapper | Class name of the content wrapper element |
| p-fieldset-content | Class name of the content element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| fieldset.background | --p-fieldset-background | Background of root |
| fieldset.border.color | --p-fieldset-border-color | Border color of root |
| fieldset.border.radius | --p-fieldset-border-radius | Border radius of root |
| fieldset.color | --p-fieldset-color | Color of root |
| fieldset.padding | --p-fieldset-padding | Padding of root |
| fieldset.transition.duration | --p-fieldset-transition-duration | Transition duration of root |
| fieldset.legend.background | --p-fieldset-legend-background | Background of legend |
| fieldset.legend.hover.background | --p-fieldset-legend-hover-background | Hover background of legend |
| fieldset.legend.color | --p-fieldset-legend-color | Color of legend |
| fieldset.legend.hover.color | --p-fieldset-legend-hover-color | Hover color of legend |
| fieldset.legend.border.radius | --p-fieldset-legend-border-radius | Border radius of legend |
| fieldset.legend.border.width | --p-fieldset-legend-border-width | Border width of legend |
| fieldset.legend.border.color | --p-fieldset-legend-border-color | Border color of legend |
| fieldset.legend.padding | --p-fieldset-legend-padding | Padding of legend |
| fieldset.legend.gap | --p-fieldset-legend-gap | Gap of legend |
| fieldset.legend.font.weight | --p-fieldset-legend-font-weight | Font weight of legend |
| fieldset.legend.font.size | --p-fieldset-legend-font-size | Font size of legend |
| fieldset.legend.focus.ring.width | --p-fieldset-legend-focus-ring-width | Focus ring width of legend |
| fieldset.legend.focus.ring.style | --p-fieldset-legend-focus-ring-style | Focus ring style of legend |
| fieldset.legend.focus.ring.color | --p-fieldset-legend-focus-ring-color | Focus ring color of legend |
| fieldset.legend.focus.ring.offset | --p-fieldset-legend-focus-ring-offset | Focus ring offset of legend |
| fieldset.legend.focus.ring.shadow | --p-fieldset-legend-focus-ring-shadow | Focus ring shadow of legend |
| fieldset.toggle.icon.color | --p-fieldset-toggle-icon-color | Color of toggle icon |
| fieldset.toggle.icon.hover.color | --p-fieldset-toggle-icon-hover-color | Hover color of toggle icon |
| fieldset.content.padding | --p-fieldset-content-padding | Padding of content |
