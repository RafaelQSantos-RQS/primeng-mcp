---
title: "Angular Accordion Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/accordion.md"
description: "Accordion groups a collection of contents in tabs."
---

# Angular Accordion Component

Accordion groups a collection of contents in tabs.

## Accessibility

Screen Reader Accordion header elements have a button role and use aria-controls to define the id of the content section along with aria-expanded for the visibility state. The value to read a header element defaults to the value of the header property and can be customized by defining an aria-label or aria-labelledby property. Each header has a heading role, for which the level is customized by headerAriaLevel and has a default level of 2 as per W3C specifications. Disabled accordions headers use aria-disabled and are excluded from the keybord navigation. The content uses region role, defines an id that matches the aria-controls of the header and aria-labelledby referring to the id of the header. Header Keyboard Support

## Basic

A simple accordion with expandable panels controlled via the value prop.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';

@Component({
    template: `
        <div class="flex justify-center">
            <p-accordion value="0" class="w-full max-w-md">
                <p-accordion-panel value="0">
                    <p-accordion-header>What is this service about?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="1">
                    <p-accordion-header>Is my data secure?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="2">
                    <p-accordion-header>Can I upgrade or downgrade my plan later?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                    </p-accordion-content>
                </p-accordion-panel>
            </p-accordion>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule]
})
export class AccordionBasicDemo {}
```

## Controlled

Control the active panel state with value and valueChange .

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="w-full max-w-md space-y-4">
                <div class="flex gap-2 justify-center">
                    <button type="button" pButton (click)="active = '0'" [severity]="active === '0' ? 'primary' : 'secondary'">Panel 1</button>
                    <button type="button" pButton (click)="active = '1'" [severity]="active === '1' ? 'primary' : 'secondary'">Panel 2</button>
                    <button type="button" pButton (click)="active = '2'" [severity]="active === '2' ? 'primary' : 'secondary'">Panel 3</button>
                    <button type="button" pButton (click)="active = null" severity="danger">Close All</button>
                </div>
                <p-accordion [(value)]="active">
                    <p-accordion-panel value="0">
                        <p-accordion-header>What is this service about?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="1">
                        <p-accordion-header>Is my data secure?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="2">
                        <p-accordion-header>Can I upgrade or downgrade my plan later?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                </p-accordion>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule, ButtonModule]
})
export class AccordionControlledDemo {
    active: string | null = '0';
}
```

## Disabled

Set disabled on an AccordionPanel to prevent user interaction with that panel.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="w-full max-w-md space-y-8">
                <p-accordion>
                    <p-accordion-panel value="0" [disabled]="true">
                        <p-accordion-header>How do I reset my password?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">You can reset your password by clicking the "Forgot password?" link on the login page. We'll send a password reset link to your registered email address.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="1" [disabled]="true">
                        <p-accordion-header>Do you offer team accounts?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Yes. Our Team and Business plans are designed for collaboration. You can invite team members, assign roles, and manage permissions easily from your dashboard.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                </p-accordion>
                <p-accordion>
                    <p-accordion-panel value="0">
                        <p-accordion-header>What happens if I exceed my usage limit?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">If you go over your plan limits (e.g., storage or API requests), you'll receive a notification. You can either upgrade your plan or wait until the next billing cycle resets.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="1" [disabled]="true">
                        <p-accordion-header>Is there a mobile app available?</p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Yes, we offer both iOS and Android apps so you can manage your account and stay connected on the go.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                </p-accordion>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule]
})
export class AccordionDisabledDemo {}
```

## Dynamic

AccordionPanel can be generated dynamically using the standard &#64;for block.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';

@Component({
    template: `
        <p-accordion [value]="['0']">
            @for (tab of tabs; track tab.title) {
                <p-accordion-panel [value]="tab.value">
                    <p-accordion-header>{{ tab.title }}</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">{{ tab.content }}</p>
                    </p-accordion-content>
                </p-accordion-panel>
            }
        </p-accordion>
    `,
    standalone: true,
    imports: [AccordionModule]
})
export class AccordionDynamicDemo {
    tabs = [
        { title: 'Title 1', content: 'Content 1', value: '0' },
        { title: 'Title 2', content: 'Content 2', value: '1' },
        { title: 'Title 3', content: 'Content 3', value: '2' }
    ];
}
```

## Indicator

The toggleicon template customizes the indicator. It can switch between two icons using the active context variable, or render a single icon that animates purely with CSS by targeting the header's data-p attribute.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';
import { Minus } from '@primeicons/angular/minus';
import { Plus } from '@primeicons/angular/plus';
import { ChevronDown } from '@primeicons/angular/chevron-down';

@Component({
    template: `
        <div class="flex flex-col gap-8">
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">Match open / closed</h3>
                <p-accordion class="block max-w-md mx-auto">
                    <p-accordion-panel value="1">
                        <p-accordion-header>
                            <ng-template #toggleicon let-active="active">
                                @if (active) {
                                    <svg data-p-icon="minus" />
                                } @else {
                                    <svg data-p-icon="plus" />
                                }
                            </ng-template>
                            What is this service about?
                        </p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">
                                This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics. Whether you're working solo or in a team, it's built to scale with your
                                needs.
                            </p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="2">
                        <p-accordion-header>
                            <ng-template #toggleicon let-active="active">
                                @if (active) {
                                    <svg data-p-icon="minus" />
                                } @else {
                                    <svg data-p-icon="plus" />
                                }
                            </ng-template>
                            Is my data secure?
                        </p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="3">
                        <p-accordion-header>
                            <ng-template #toggleicon let-active="active">
                                @if (active) {
                                    <svg data-p-icon="minus" />
                                } @else {
                                    <svg data-p-icon="plus" />
                                }
                            </ng-template>
                            Can I upgrade or downgrade my plan later?
                        </p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                </p-accordion>
            </div>
            <div>
                <h3 class="text-sm! font-medium! text-surface-500! mb-2">CSS-only with data attributes</h3>
                <p-accordion class="block max-w-md mx-auto">
                    <p-accordion-panel value="1">
                        <p-accordion-header>
                            <ng-template #toggleicon>
                                <svg data-p-icon="chevron-down" class="transition-transform duration-200 in-data-[p=active]:rotate-180" />
                            </ng-template>
                            What is this service about?
                        </p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">
                                This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics. Whether you're working solo or in a team, it's built to scale with your
                                needs.
                            </p>
                        </p-accordion-content>
                    </p-accordion-panel>
                    <p-accordion-panel value="2">
                        <p-accordion-header>
                            <ng-template #toggleicon>
                                <svg data-p-icon="chevron-down" class="transition-transform duration-200 in-data-[p=active]:rotate-180" />
                            </ng-template>
                            Is my data secure?
                        </p-accordion-header>
                        <p-accordion-content>
                            <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                        </p-accordion-content>
                    </p-accordion-panel>
                </p-accordion>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule, Minus, Plus, ChevronDown]
})
export class AccordionIndicatorDemo {}
```

## Multiple

By default, a single panel is open at a time. Enable multiple to allow multiple panels to stay open.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';

@Component({
    template: `
        <div class="flex justify-center">
            <p-accordion [multiple]="true" class="w-full max-w-md">
                <p-accordion-panel value="0">
                    <p-accordion-header>What is this service about?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">
                            This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics. Whether you're working solo or in a team, it's built to scale with your needs.
                        </p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="1">
                    <p-accordion-header>Is my data secure?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="2">
                    <p-accordion-header>Can I upgrade or downgrade my plan later?</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                    </p-accordion-content>
                </p-accordion-panel>
            </p-accordion>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule]
})
export class AccordionMultipleDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';

@Component({
    template: `
        <div class="flex justify-center">
            <p-accordion class="w-full max-w-md">
                <p-accordion-panel value="0">
                    <p-accordion-header>About the role and team</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">
                            Join our product team as a Senior Frontend Engineer to shape the next generation of our design system. You will work closely with designers and product managers to deliver polished, accessible interfaces.
                        </p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="1">
                    <p-accordion-header>Day-to-day responsibilities</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Build reusable components, drive technical decisions across the frontend stack, mentor junior engineers, and contribute to architecture reviews and engineering best practices.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="2">
                    <p-accordion-header>What we are looking for</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">5+ years of experience with Angular and TypeScript, strong understanding of modern CSS, and a passion for building accessible, performant user interfaces at scale.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="3">
                    <p-accordion-header>Compensation and benefits</p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Competitive salary, equity, fully remote setup, generous paid time off, learning stipend, and a yearly company offsite to meet the team in person.</p>
                    </p-accordion-content>
                </p-accordion-panel>
            </p-accordion>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule]
})
export class AccordionPreviewDemo {}
```

## With RadioButton

RadioButton component can be used to group multiple AccordionPanel components.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AccordionModule } from 'primeng/accordion';
import { RadioButtonModule } from 'primeng/radiobutton';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="w-full max-w-sm">
                <p-accordion [(value)]="selected" class="w-full block border border-surface-200 dark:border-surface-700 rounded-md divide-y divide-surface-200 dark:divide-surface-700">
                    @for (item of items; track item.value) {
                        <p-accordion-panel [value]="item.value">
                            <p-accordion-header>
                                <ng-template #toggleicon>
                                    <span></span>
                                </ng-template>
                                <span class="flex items-center gap-3 w-full">
                                    <p-radiobutton [value]="item.value" [(ngModel)]="selected" />
                                    <span class="flex items-center justify-between flex-1">
                                        <span class="font-semibold text-base">{{ item.label }}</span>
                                        <span class="font-semibold text-base">{{ item.price }}</span>
                                    </span>
                                </span>
                            </p-accordion-header>
                            <p-accordion-content>
                                <p class="m-0 text-surface-500 pl-8">{{ item.description }}</p>
                            </p-accordion-content>
                        </p-accordion-panel>
                    }
                </p-accordion>
                <button type="button" pButton class="w-full mt-4" size="large">Buy Now for {{ selectedPrice }}</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule, RadioButtonModule, ButtonModule, FormsModule]
})
export class AccordionRadioButtonDemo {
    items = [
        {
            label: 'Starter Plan',
            description: 'Perfect for individuals getting started. Includes access to core components and community support.',
            value: '0',
            price: '$99'
        },
        {
            label: 'Growth Plan',
            description: 'Ideal for freelancers and small teams. Unlocks advanced UI components and priority email support.',
            value: '1',
            price: '$249'
        },
        {
            label: 'Scale Plan',
            description: 'Best for growing businesses. Includes all features, early access to new releases, and Slack/Jira support.',
            value: '2',
            price: '$499'
        }
    ];

    selected: string = '0';

    get selectedPrice(): string {
        return this.items.find((item) => item.value === this.selected)?.price ?? '';
    }
}
```

## Template

Create accordion panels dynamically by iterating over a data source to keep structure consistent and reusable.

```typescript
import { Component } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';
import { Minus } from '@primeicons/angular/minus';
import { Plus } from '@primeicons/angular/plus';
import { QuestionCircle } from '@primeicons/angular/question-circle';
import { Lock } from '@primeicons/angular/lock';
import { CreditCard } from '@primeicons/angular/credit-card';

@Component({
    template: `
        <div class="flex justify-center">
            <p-accordion class="w-full max-w-md">
                <p-accordion-panel value="0">
                    <p-accordion-header>
                        <ng-template #toggleicon let-active="active">
                            @if (active) {
                                <svg data-p-icon="minus" />
                            } @else {
                                <svg data-p-icon="plus" />
                            }
                        </ng-template>
                        <span class="flex items-center gap-4">
                            <svg data-p-icon="question-circle" class="text-yellow-500" />
                            <span class="font-medium">What is this service about?</span>
                        </span>
                    </p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">
                            This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics. Whether you're working solo or in a team, it's built to scale with your needs.
                        </p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="1">
                    <p-accordion-header>
                        <ng-template #toggleicon let-active="active">
                            @if (active) {
                                <svg data-p-icon="minus" />
                            } @else {
                                <svg data-p-icon="plus" />
                            }
                        </ng-template>
                        <span class="flex items-center gap-4">
                            <svg data-p-icon="lock" class="text-blue-500" />
                            <span class="font-medium">Is my data secure?</span>
                        </span>
                    </p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                    </p-accordion-content>
                </p-accordion-panel>
                <p-accordion-panel value="2">
                    <p-accordion-header>
                        <ng-template #toggleicon let-active="active">
                            @if (active) {
                                <svg data-p-icon="minus" />
                            } @else {
                                <svg data-p-icon="plus" />
                            }
                        </ng-template>
                        <span class="flex items-center gap-4">
                            <svg data-p-icon="credit-card" class="text-green-500" />
                            <span class="font-medium">Can I upgrade or downgrade my plan later?</span>
                        </span>
                    </p-accordion-header>
                    <p-accordion-content>
                        <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                    </p-accordion-content>
                </p-accordion-panel>
            </p-accordion>
        </div>
    `,
    standalone: true,
    imports: [AccordionModule, Minus, Plus, QuestionCircle, Lock, CreditCard]
})
export class AccordionTemplateDemo {}
```

## Trigger

The trigger can be tailored per panel; the toggleicon template animates a custom icon via the header's data-p attribute, while a fully custom header lets the toggle be rendered as any element, such as a Button , by driving the active state with the value model.

```typescript
import { Component, signal } from '@angular/core';
import { AccordionModule } from 'primeng/accordion';
import { ButtonModule } from 'primeng/button';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <p-accordion [(value)]="value" [multiple]="true" class="block max-w-md mx-auto">
            <p-accordion-panel value="1">
                <p-accordion-header>
                    <ng-template #toggleicon>
                        <svg data-p-icon="plus" class="transition-transform ease-out in-data-[p=active]:rotate-45" />
                    </ng-template>
                    What is this service about?
                </p-accordion-header>
                <p-accordion-content>
                    <p class="m-0 text-sm">
                        This service helps you manage your projects more efficiently by offering real-time collaboration, task tracking, and powerful analytics. Whether you're working solo or in a team, it's built to scale with your needs.
                    </p>
                </p-accordion-content>
            </p-accordion-panel>
            <p-accordion-panel value="2">
                <p-accordion-header>
                    <ng-template #toggleicon><span class="hidden"></span></ng-template>
                    <div class="flex items-center justify-between gap-4 -mx-4 -my-2 px-4 py-2 grow" (click)="$event.stopPropagation()">
                        <span class="text-sm font-semibold">Is my data secure?</span>
                        <button type="button" pButton variant="outlined" size="small" (click)="toggle('2'); $event.stopPropagation()">{{ isActive('2') ? 'Hide' : 'Show' }}</button>
                    </div>
                </p-accordion-header>
                <p-accordion-content>
                    <p class="m-0 text-sm">Yes. We use end-to-end encryption and follow industry best practices to ensure your data is protected. Your information is stored on secure servers and regularly backed up.</p>
                </p-accordion-content>
            </p-accordion-panel>
            <p-accordion-panel value="3">
                <p-accordion-header>
                    <ng-template #toggleicon><span class="hidden"></span></ng-template>
                    <span class="inline-flex items-center gap-2">
                        <svg data-p-icon="plus" class="transition-transform ease-out in-data-[p=active]:rotate-45" />
                        Can I upgrade or downgrade my plan later?
                    </span>
                </p-accordion-header>
                <p-accordion-content>
                    <p class="m-0 text-sm">Absolutely. You can change your subscription plan at any time from your account settings. Changes take effect immediately, and any billing adjustments are handled automatically.</p>
                </p-accordion-content>
            </p-accordion-panel>
        </p-accordion>
    `,
    standalone: true,
    imports: [AccordionModule, ButtonModule, Plus]
})
export class AccordionTriggerDemo {
    value = signal<string[]>([]);

    isActive(panel: string): boolean {
        return this.value().includes(panel);
    }

    toggle(panel: string) {
        this.value.update((current) => (current.includes(panel) ? current.filter((v) => v !== panel) : [...current, panel]));
    }
}
```

## Accordion

Accordion groups a collection of contents in tabs.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| value | string \| number \| (string \| number)[] \| null \| undefined | undefined | Value of the active tab. |
| multiple | boolean | false | When enabled, multiple tabs can be activated at the same time. |
| expandIcon | string \| undefined | - | Icon of a collapsed tab. |
| collapseIcon | string \| undefined | - | Icon of an expanded tab. |
| selectOnFocus | boolean | false | When enabled, the focused tab is activated. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, AccordionPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onClose | event: AccordionTabCloseEvent | Callback to invoke when an active tab is collapsed by clicking on the header. |
| onOpen | event: AccordionTabOpenEvent | Callback to invoke when a tab gets expanded. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-accordion | Class name of the root element |
| p-accordioncontent | Class name of the content wrapper |
| p-accordioncontent-content | Class name of the content |
| p-accordionheader | Class name of the header |
| p-accordionheader-toggle-icon | Class name of the toggle icon |
| p-accordionpanel | Class name of the panel |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| accordion.transition.duration | --p-accordion-transition-duration | Transition duration of root |
| accordion.panel.border.width | --p-accordion-panel-border-width | Border width of panel |
| accordion.panel.border.color | --p-accordion-panel-border-color | Border color of panel |
| accordion.header.color | --p-accordion-header-color | Color of header |
| accordion.header.hover.color | --p-accordion-header-hover-color | Hover color of header |
| accordion.header.active.color | --p-accordion-header-active-color | Active color of header |
| accordion.header.active.hover.color | --p-accordion-header-active-hover-color | Active hover color of header |
| accordion.header.padding | --p-accordion-header-padding | Padding of header |
| accordion.header.font.weight | --p-accordion-header-font-weight | Font weight of header |
| accordion.header.font.size | --p-accordion-header-font-size | Font size of header |
| accordion.header.border.radius | --p-accordion-header-border-radius | Border radius of header |
| accordion.header.border.width | --p-accordion-header-border-width | Border width of header |
| accordion.header.border.color | --p-accordion-header-border-color | Border color of header |
| accordion.header.background | --p-accordion-header-background | Background of header |
| accordion.header.hover.background | --p-accordion-header-hover-background | Hover background of header |
| accordion.header.active.background | --p-accordion-header-active-background | Active background of header |
| accordion.header.active.hover.background | --p-accordion-header-active-hover-background | Active hover background of header |
| accordion.header.focus.ring.width | --p-accordion-header-focus-ring-width | Focus ring width of header |
| accordion.header.focus.ring.style | --p-accordion-header-focus-ring-style | Focus ring style of header |
| accordion.header.focus.ring.color | --p-accordion-header-focus-ring-color | Focus ring color of header |
| accordion.header.focus.ring.offset | --p-accordion-header-focus-ring-offset | Focus ring offset of header |
| accordion.header.focus.ring.shadow | --p-accordion-header-focus-ring-shadow | Focus ring shadow of header |
| accordion.header.toggle.icon.color | --p-accordion-header-toggle-icon-color | Toggle icon color of header |
| accordion.header.toggle.icon.hover.color | --p-accordion-header-toggle-icon-hover-color | Toggle icon hover color of header |
| accordion.header.toggle.icon.active.color | --p-accordion-header-toggle-icon-active-color | Toggle icon active color of header |
| accordion.header.toggle.icon.active.hover.color | --p-accordion-header-toggle-icon-active-hover-color | Toggle icon active hover color of header |
| accordion.header.first.top.border.radius | --p-accordion-header-first-top-border-radius | First top border radius of header |
| accordion.header.first.border.width | --p-accordion-header-first-border-width | First border width of header |
| accordion.header.last.bottom.border.radius | --p-accordion-header-last-bottom-border-radius | Last bottom border radius of header |
| accordion.header.last.active.bottom.border.radius | --p-accordion-header-last-active-bottom-border-radius | Last active bottom border radius of header |
| accordion.content.border.width | --p-accordion-content-border-width | Border width of content |
| accordion.content.border.color | --p-accordion-content-border-color | Border color of content |
| accordion.content.background | --p-accordion-content-background | Background of content |
| accordion.content.color | --p-accordion-content-color | Color of content |
| accordion.content.padding | --p-accordion-content-padding | Padding of content |
