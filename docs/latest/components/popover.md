---
title: "Angular Popover Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/popover.md"
description: "Popover is a container component that can overlay other components on page."
---

# Angular Popover Component

Popover is a container component that can overlay other components on page.

## Accessibility

Screen Reader Popover component uses dialog role and since any attribute is passed to the root element you may define attributes like aria-label or aria-labelledby to describe the popup contents. In addition aria-modal is added since focus is kept within the popup. It is recommended to use a trigger component that can be accessed with keyboard such as a button, if not adding tabIndex would be necessary. Popover adds aria-expanded state attribute and aria-controls to the trigger so that the relation between the trigger and the popup is defined. Popover Keyboard Support When the popup gets opened, the first focusable element receives the focus and this can be customized by adding autofocus to an element within the popup. Key Function tab Moves focus to the next the focusable element within the popup. shift + tab Moves focus to the previous the focusable element within the popup. escape Closes the popup and moves focus to the trigger. Close Button Keyboard Support Key Function enter Closes the popup and moves focus to the trigger. space Closes the popup and moves focus to the trigger.

## Basic

Popover is accessed via its reference and visibility is controlled using toggle , show and hide methods with an event of the target.

```typescript
import { Component } from '@angular/core';
import { Popover, PopoverModule } from 'primeng/popover';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton severity="secondary" variant="outlined" (click)="op.toggle($event)">Show Popover</button>
            <p-popover #op>
                <div class="max-w-72 w-full">
                    <div class="flex items-center justify-between">
                        <span class="font-semibold text-sm">Create a New Workspace</span>
                    </div>
                    <p class="text-sm text-muted-color mt-2 mb-0!">Name your workspace to get started. You can always change this later.</p>
                    <input pInputText placeholder="Workspace Name" class="mt-3 w-full" />
                    <div class="flex items-center justify-between mt-4">
                        <span class="text-xs text-surface-500 dark:text-surface-400">Lowercase letters and dashes only</span>
                        <div class="flex items-center gap-2">
                            <button type="button" pButton severity="secondary" variant="outlined" size="small" (click)="op.hide()">Cancel</button>
                            <button type="button" pButton size="small" (click)="op.hide()">Create</button>
                        </div>
                    </div>
                </div>
            </p-popover>
        </div>
    `,
    standalone: true,
    imports: [PopoverModule, ButtonModule, InputTextModule]
})
export class PopoverBasicDemo {}
```

## Controlled

Popover visibility can be controlled from outside by accessing the component with a template reference variable and calling its show , hide and toggle methods. Pass the trigger element as the second argument to keep the overlay anchored to it.

```typescript
import { Component } from '@angular/core';
import { Popover, PopoverModule } from 'primeng/popover';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex gap-4 justify-center items-center">
            <button type="button" pButton (click)="op.toggle($event, trigger)">Show Popover</button>
            <button #trigger type="button" pButton severity="secondary" variant="outlined" (click)="op.toggle($event)">Popover Trigger</button>
            <p-popover #op>
                <div class="max-w-72 w-full">
                    <div class="flex items-center justify-between">
                        <span class="font-semibold text-sm">Create a New Workspace</span>
                    </div>
                    <p class="text-sm text-muted-color mt-2 mb-0!">Name your workspace to get started. You can always change this later.</p>
                    <input pInputText placeholder="Workspace Name" class="mt-3 w-full" />
                    <div class="flex items-center justify-between mt-4">
                        <span class="text-xs text-surface-500 dark:text-surface-400">Lowercase letters and dashes only</span>
                        <div class="flex items-center gap-2">
                            <button type="button" pButton severity="secondary" variant="outlined" size="small" (click)="op.hide()">Cancel</button>
                            <button type="button" pButton size="small" (click)="op.hide()">Create</button>
                        </div>
                    </div>
                </div>
            </p-popover>
        </div>
    `,
    standalone: true,
    imports: [PopoverModule, ButtonModule, InputTextModule]
})
export class PopoverControlledDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { PopoverModule } from 'primeng/popover';
import { TagModule } from 'primeng/tag';
import { ButtonModule } from 'primeng/button';
import { Send } from '@primeicons/angular/send';
import { ArrowRight } from '@primeicons/angular/arrow-right';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton severity="secondary" variant="outlined" (click)="op.toggle($event)">Track Flight</button>
            <p-popover #op styleClass="p-2">
                <div class="w-80">
                    <div class="flex items-start justify-between">
                        <div class="flex flex-col">
                            <span class="text-base font-semibold">British Airways</span>
                            <span class="text-muted-color text-xs mt-0.5 tabular-nums">BA 178</span>
                        </div>
                        <p-tag severity="success">ON TIME</p-tag>
                    </div>
                    <div class="flex items-start justify-between gap-3 mt-4">
                        <div class="flex flex-col gap-1.5">
                            <span class="text-2xl font-bold leading-none tracking-tight">LHR</span>
                            <span class="text-muted-color text-xs">London</span>
                            <span class="text-sm font-semibold tabular-nums">14:30</span>
                        </div>
                        <div class="flex-1 flex flex-col items-center pt-2">
                            <div class="w-full flex items-center gap-2">
                                <span class="flex-1 border-t border-dashed border-surface-300 dark:border-surface-700"></span>
                                <svg data-p-icon="send" [size]="14" class="text-muted-color shrink-0 rotate-90"></svg>
                                <span class="flex-1 border-t border-dashed border-surface-300 dark:border-surface-700"></span>
                            </div>
                            <span class="text-muted-color text-xs mt-2 tabular-nums">8h 15m</span>
                        </div>
                        <div class="flex flex-col items-end gap-1.5">
                            <span class="text-2xl font-bold leading-none tracking-tight">JFK</span>
                            <span class="text-muted-color text-xs">New York</span>
                            <span class="text-sm font-semibold tabular-nums">18:45</span>
                        </div>
                    </div>
                    <div class="flex items-center justify-between mt-4 border-surface-200 dark:border-surface-700">
                        <span class="text-xs text-muted-color">Gate B12 · Terminal 5</span>
                        <button type="button" pButton size="small" variant="link" class="ml-auto">
                            Track flight
                            <svg data-p-icon="arrow-right" [size]="12"></svg>
                        </button>
                    </div>
                </div>
            </p-popover>
        </div>
    `,
    standalone: true,
    imports: [PopoverModule, TagModule, ButtonModule, Send, ArrowRight]
})
export class PopoverPreviewDemo {}
```

## Popover

Popover is a container component that can overlay other components on page.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, PopoverPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| dismissable | boolean | - | Enables to hide the overlay when outside is clicked. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| styleClass | string \| undefined | - | Style class of the component. |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'body' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| ariaCloseLabel | string \| undefined | - | Aria label of the close icon. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| focusOnShow | boolean | - | When enabled, first button receives focus on show. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onShow | value: any | Callback to invoke when an overlay becomes visible. |
| onHide | value: any | Callback to invoke when an overlay gets hidden. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| content | TemplateRef<PopoverContentTemplateContext> \| undefined | Custom content template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| toggle | event: any, target: any | void | Toggles the visibility of the panel. |
| show | event: any, target: any | void | Displays the panel. |
| hide |  | void | Hides the panel. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| popover.background | --p-popover-background | Background of root |
| popover.border.color | --p-popover-border-color | Border color of root |
| popover.color | --p-popover-color | Color of root |
| popover.border.radius | --p-popover-border-radius | Border radius of root |
| popover.shadow | --p-popover-shadow | Shadow of root |
| popover.gutter | --p-popover-gutter | Gutter of root |
| popover.arrow.offset | --p-popover-arrow-offset | Arrow offset of root |
| popover.content.padding | --p-popover-content-padding | Padding of content |
