---
title: "Angular Inplace Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inplace.md"
description: "Inplace provides an easy to do editing and display at the same time where clicking the output displays the actual content."
---

# Angular Inplace Component

Inplace provides an easy to do editing and display at the same time where clicking the output displays the actual content.

## Accessibility

Screen Reader Inplace component defines aria-live as "polite" by default, since any valid attribute is passed to the main container aria roles and attributes of the root element can be customized easily. Display element uses button role in view mode by default, displayProps can be used for customizations like adding aria-label or aria-labelledby attributes to describe the content of the view mode or even overriding the default role. Closable inplace components displays a button with an aria-label that refers to the aria.close property of the locale API by default, you may use closeButtonProps to customize the element and override the default aria-label . View Mode Keyboard Support Key Function enter Switches to content. Close Button Keyboard Support Key Function enter Switches to display. space Switches to display.

## Basic

Inplace component requires display and content templates to define the content of each state.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InplaceModule } from 'primeng/inplace';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { AutoFocusModule } from 'primeng/autofocus';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="max-w-3xs mx-auto w-full">
            <div class="font-mono uppercase text-xs opacity-50 pl-2 mb-1">Name</div>
            <p-inplace [pt]="{ display: 'w-full' }">
                <ng-template #display>
                    <span class="w-full text-sm">John Doe</span>
                </ng-template>
                <ng-template #content let-closeCallback="closeCallback">
                    <p-inputgroup>
                        <input pInputText [(ngModel)]="name" placeholder="Enter name" class="flex-1" fluid [pAutoFocus]="true" />
                        <p-inputgroup-addon>
                            <button pButton text severity="success">
                                <svg data-p-icon="check"></svg>
                            </button>
                        </p-inputgroup-addon>
                        <p-inputgroup-addon>
                            <button pButton text severity="danger" (click)="closeCallback($event)">
                                <svg data-p-icon="times"></svg>
                            </button>
                        </p-inputgroup-addon>
                    </p-inputgroup>
                </ng-template>
            </p-inplace>
        </div>
    `,
    standalone: true,
    imports: [InplaceModule, InputGroupModule, InputGroupAddonModule, ButtonModule, InputTextModule, AutoFocusModule, FormsModule, Check, Times]
})
export class InplaceBasicDemo {
    name: string = '';
}
```

## Controlled

The active state can be controlled programmatically with the active property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InplaceModule } from 'primeng/inplace';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { AutoFocusModule } from 'primeng/autofocus';

@Component({
    template: `
        <div class="max-w-3xs mx-auto w-full">
            <div class="font-mono uppercase text-xs opacity-50 pl-2 mb-1">Name</div>
            <p-inplace #ip [pt]="{ display: 'w-full' }">
                <ng-template #display>
                    <span class="w-full text-sm">John Doe</span>
                </ng-template>
                <ng-template #content>
                    <input pInputText [(ngModel)]="name" placeholder="Enter name" class="flex-1" fluid [pAutoFocus]="true" />
                </ng-template>
            </p-inplace>
            <div class="flex items-center gap-2 mt-2">
                <button pButton class="flex-1" variant="outlined" severity="secondary" (click)="ip.active() ? ip.deactivate() : ip.activate()">{{ ip.active() ? 'Cancel' : 'Edit Name' }}</button>
                @if (ip.active()) {
                    <button pButton class="flex-1" variant="outlined" fluid (click)="ip.deactivate()">Save</button>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [InplaceModule, ButtonModule, InputTextModule, AutoFocusModule, FormsModule]
})
export class InplaceControlledDemo {
    name: string = '';
}
```

## Image

Any content such as an image can be placed inside an Inplace.

```typescript
import { Component } from '@angular/core';
import { InplaceModule } from 'primeng/inplace';
import { Photo } from '@/domain/photo';
import { Image } from '@primeicons/angular/image';

@Component({
    template: `
        <p-inplace>
            <ng-template #display>
                <span class="inline-flex items-center gap-2">
                    <svg data-p-icon="image" style="vertical-align: middle"></svg>
                    <span class="ml-2 text-sm">View Photo</span>
                </span>
            </ng-template>
            <ng-template #content>
                <img class="w-full sm:w-80 shadow-md" src="https://primefaces.org/cdn/primeng/images/demo/galleria/galleria5.jpg" alt="Nature" />
            </ng-template>
        </p-inplace>
    `,
    standalone: true,
    imports: [InplaceModule, Image]
})
export class InplaceImageDemo {}
```

## Input

The closeCallback switches the state back to display mode when called from an event.

```typescript
import { Component } from '@angular/core';
import { InplaceModule } from 'primeng/inplace';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { AutoFocusModule } from 'primeng/autofocus';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <p-inplace>
            <ng-template #display>
                <span class="text-sm">Click to Edit</span>
            </ng-template>
            <ng-template #content let-closeCallback="closeCallback">
                <span class="inline-flex gap-2">
                    <input type="text" pInputText [pAutoFocus]="true" />
                    <button type="button" pButton (click)="closeCallback($event)" text severity="danger">
                        <svg data-p-icon="times"></svg>
                    </button>
                </span>
            </ng-template>
        </p-inplace>
    `,
    standalone: true,
    imports: [InplaceModule, ButtonModule, InputTextModule, AutoFocusModule, Times]
})
export class InplaceInputDemo {}
```

## Lazy

Using the onActivate event, data can be loaded in a lazy manner before displaying it in a table.

```typescript
import { Component, inject } from '@angular/core';
import { InplaceModule } from 'primeng/inplace';
import { TableModule } from 'primeng/table';
import { ProductService } from '@/service/productservice';
import { Product } from '@/domain/product';

@Component({
    template: `
        <p-inplace (onActivate)="loadData()">
            <ng-template #display>
                <span class="text-sm">View Data</span>
            </ng-template>
            <ng-template #content>
                <p-table [value]="products">
                    <ng-template #header>
                        <tr>
                            <th>Code</th>
                            <th>Name</th>
                            <th>Category</th>
                            <th>Quantity</th>
                        </tr>
                    </ng-template>
                    <ng-template #body let-product>
                        <tr>
                            <td>{{ product.code }}</td>
                            <td>{{ product.name }}</td>
                            <td>{{ product.category }}</td>
                            <td>{{ product.quantity }}</td>
                        </tr>
                    </ng-template>
                </p-table>
            </ng-template>
        </p-inplace>
    `,
    standalone: true,
    imports: [InplaceModule, TableModule],
    providers: [ProductService]
})
export class InplaceLazyDemo {
    private productService = inject(ProductService);

    products: Product[] | undefined;

    loadData() {
        this.productService.getProductsMini().then((products) => (this.products = products));
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { DividerModule } from 'primeng/divider';
import { InplaceModule } from 'primeng/inplace';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { AutoFocusModule } from 'primeng/autofocus';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="max-w-3xs mx-auto w-full">
            <div class="text-sm font-medium pl-2">Profile</div>
            <p class="text-sm text-surface-500 mt-1 pl-2">Update your profile settings.</p>
            <p-divider />
            <div class="space-y-4">
                <div>
                    <div class="font-mono uppercase text-xs opacity-50 pl-2 mb-1">Name</div>
                    <p-inplace [pt]="{ display: 'w-full' }">
                        <ng-template #display>
                            <span class="w-full text-sm">John Doe</span>
                        </ng-template>
                        <ng-template #content let-closeCallback="closeCallback">
                            <p-inputgroup>
                                <input pInputText [(ngModel)]="name" placeholder="Enter name" class="flex-1" fluid [pAutoFocus]="true" />
                                <p-inputgroup-addon>
                                    <button pButton text severity="success">
                                        <svg data-p-icon="check"></svg>
                                    </button>
                                </p-inputgroup-addon>
                                <p-inputgroup-addon>
                                    <button pButton text severity="danger" (click)="closeCallback($event)">
                                        <svg data-p-icon="times"></svg>
                                    </button>
                                </p-inputgroup-addon>
                            </p-inputgroup>
                        </ng-template>
                    </p-inplace>
                </div>
                <div>
                    <div class="font-mono uppercase text-xs opacity-50 pl-2 mb-1">Email</div>
                    <p-inplace [pt]="{ display: 'w-full' }">
                        <ng-template #display>
                            <span class="w-full text-sm">john.doe&#64;example.com</span>
                        </ng-template>
                        <ng-template #content let-closeCallback="closeCallback">
                            <p-inputgroup>
                                <input pInputText [(ngModel)]="email" placeholder="Enter email" class="flex-1" fluid [pAutoFocus]="true" />
                                <p-inputgroup-addon>
                                    <button pButton text severity="success">
                                        <svg data-p-icon="check"></svg>
                                    </button>
                                </p-inputgroup-addon>
                                <p-inputgroup-addon>
                                    <button pButton text severity="danger" (click)="closeCallback($event)">
                                        <svg data-p-icon="times"></svg>
                                    </button>
                                </p-inputgroup-addon>
                            </p-inputgroup>
                        </ng-template>
                    </p-inplace>
                </div>
                <div>
                    <div class="font-mono uppercase text-xs opacity-50 pl-2 mb-1">Password</div>
                    <p-inplace [pt]="{ display: 'w-full' }">
                        <ng-template #display>
                            <span class="w-full text-sm">********</span>
                        </ng-template>
                        <ng-template #content let-closeCallback="closeCallback">
                            <p-inputgroup>
                                <input pInputText pInputPassword [(ngModel)]="password" placeholder="Enter password" class="flex-1" fluid [mask]="true" [pAutoFocus]="true" />
                                <p-inputgroup-addon>
                                    <button pButton text severity="success">
                                        <svg data-p-icon="check"></svg>
                                    </button>
                                </p-inputgroup-addon>
                                <p-inputgroup-addon>
                                    <button pButton text severity="danger" (click)="closeCallback($event)">
                                        <svg data-p-icon="times"></svg>
                                    </button>
                                </p-inputgroup-addon>
                            </p-inputgroup>
                        </ng-template>
                    </p-inplace>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, InplaceModule, InputGroupModule, InputGroupAddonModule, ButtonModule, InputTextModule, AutoFocusModule, InputPasswordModule, FormsModule, Check, Times]
})
export class InplacePreviewDemo {
    name: string = '';

    email: string = '';

    password: string = '';
}
```

## Inplace

Inplace provides an easy to do editing and display at the same time where clicking the output displays the actual content.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InplacePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| active | WritableSignal<boolean> | - | Whether the content is displayed or not. |
| disabled | boolean | - | When present, it specifies that the element should be disabled. |
| preventClick | boolean | - | Allows to prevent clicking. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onActivate | event: Event | Callback to invoke when inplace is opened. |
| onDeactivate | event: Event | Callback to invoke when inplace is closed. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| display | TemplateRef<void> \| undefined | Custom display template. |
| content | TemplateRef<InplaceContentTemplateContext> \| undefined | Custom content template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| activate | event: Event | void | Activates the content. |
| deactivate | event: Event | void | Deactivates the content. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| display | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the display's DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| pcButton | ButtonPassThrough | Used to pass attributes to the Button component. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inplace | Class name of the root element |
| p-inplace-display | Class name of the display element |
| p-inplace-content | Class name of the content element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| inplace.padding | --p-inplace-padding | Padding of root |
| inplace.border.radius | --p-inplace-border-radius | Border radius of root |
| inplace.focus.ring.width | --p-inplace-focus-ring-width | Focus ring width of root |
| inplace.focus.ring.style | --p-inplace-focus-ring-style | Focus ring style of root |
| inplace.focus.ring.color | --p-inplace-focus-ring-color | Focus ring color of root |
| inplace.focus.ring.offset | --p-inplace-focus-ring-offset | Focus ring offset of root |
| inplace.focus.ring.shadow | --p-inplace-focus-ring-shadow | Focus ring shadow of root |
| inplace.transition.duration | --p-inplace-transition-duration | Transition duration of root |
| inplace.display.hover.background | --p-inplace-display-hover-background | Hover background of display |
| inplace.display.hover.color | --p-inplace-display-hover-color | Hover color of display |
