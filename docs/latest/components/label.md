---
title: "Angular Label Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/label.md"
description: "Label provides accessible text labels for form controls. Use the for attribute to link the label to a form control by its id."
---

# Angular Label Component

Label provides accessible text labels for form controls. Use the for attribute to link the label to a form control by its id.

## Accessibility

Screen Reader Label renders a native label element. Use the for attribute to associate it with a form control id, or wrap the form control inside the label. Keyboard Support Component does not include any interactive elements.

## Basic

An accessible label element associated with a form control.

```typescript
import { Component } from '@angular/core';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-wrap justify-center">
            <div class="flex flex-col gap-2 w-full max-w-sm">
                <label pLabel for="username">Username</label>
                <input pInputText id="username" placeholder="Enter username" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, LabelModule]
})
export class LabelBasicDemo {}
```

## Disabled

Label reflects the disabled state of an associated control automatically when the control and label are peers or when the container has data-disabled .

```typescript
import { Component } from '@angular/core';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-col items-start max-w-3xs w-full mx-auto gap-6">
            <div class="flex flex-col gap-2 w-full">
                <label pLabel for="account-id">Account ID</label>
                <input pInputText id="account-id" value="ACC-2026-0042" disabled />
            </div>
            <div [attr.data-disabled]="true" class="flex flex-col gap-2 w-full">
                <label pLabel for="api-key">API Key</label>
                <span>
                    <input pInputText id="api-key" value="sk_live_***************" disabled fluid />
                </span>
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, LabelModule]
})
export class LabelDisabledDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-col gap-2 w-full max-w-3xs mx-auto">
            <label pLabel for="username">Username</label>
            <input pInputText id="username" placeholder="Enter username" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, LabelModule]
})
export class LabelPreviewDemo {}
```

## Required

You can display required indicators in the label content while keeping the input semantics with required or aria-required .

```typescript
import { Component } from '@angular/core';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';
import { Envelope } from '@primeicons/angular/envelope';

@Component({
    template: `
        <div class="flex flex-wrap justify-center">
            <div class="flex flex-col gap-2 w-full max-w-sm">
                <label pLabel for="email" class="font-medium">
                    <svg data-p-icon="envelope" />
                    <span>Email</span>
                    <span aria-hidden="true">*</span>
                </label>
                <input pInputText id="email" type="email" placeholder="name@example.com" required aria-required="true" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, LabelModule, Envelope]
})
export class LabelRequiredDemo {}
```

## Wrapper

Label can wrap a form control to associate them implicitly without for . Useful for card-style selectable options.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-wrap justify-center">
            <div class="flex flex-col w-full max-w-3xs gap-2">
                @for (plan of plans; track plan.id) {
                    <label pLabel [class]="'flex items-start gap-3 p-3 rounded-md border cursor-pointer ' + (selected === plan.id ? 'border-primary' : 'border-surface')">
                        <p-radiobutton name="plan" [value]="plan.id" [(ngModel)]="selected" />
                        <div class="flex flex-col gap-0.5">
                            <span class="font-medium">{{ plan.name }}</span>
                            <span class="text-xs text-surface-500">{{ plan.description }}</span>
                        </div>
                    </label>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, LabelModule, FormsModule]
})
export class LabelWrapperDemo {
    selected: string = 'pro';

    plans = [
        { id: 'starter', name: 'Starter', description: 'For solo developers.' },
        { id: 'pro', name: 'Pro', description: 'For growing teams.' },
        { id: 'enterprise', name: 'Enterprise', description: 'For large organizations.' }
    ];
}
```

## Label

Label directive is an extension to standard label element with theming.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, LabelPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLLabelElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-label | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| label.gap | --p-label-gap | Gap of root |
| label.font.size | --p-label-font-size | Font size of root |
| label.font.weight | --p-label-font-weight | Font weight of root |
| label.text.color | --p-label-text-color | Text color of root |
| label.disabled.opacity | --p-label-disabled-opacity | Disabled opacity of root |
