---
title: "Angular ToggleSwitch Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/toggleswitch.md"
description: "ToggleSwitch is used to select a boolean value."
---

# Angular ToggleSwitch Component

ToggleSwitch is used to select a boolean value.

## Accessibility

Screen Reader InputSwitch component uses a hidden native checkbox element with switch role internally that is only visible to screen readers. Value to describe the component can either be provided via label tag combined with inputId prop or using ariaLabelledBy , ariaLabel props.

## Basic

Toggles a boolean setting between enabled and disabled states.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div class="flex justify-center items-center gap-2">
            <label for="switch">Off</label>
            <p-toggleswitch [(ngModel)]="checked" inputId="switch" />
            <label for="switch">On</label>
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchBasicDemo {
    checked: boolean = false;
}
```

## Customization

ToggleSwitch supports customization through the pt property. The appearance, including colors and other visual properties, can be modified by applying custom classes.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div>
            <label for="custom" class="max-w-xs mx-auto flex items-start gap-2 bg-slate-50 hover:bg-slate-100 dark:bg-slate-800 hover:dark:bg-slate-700 p-4 rounded-md transition-colors">
                <div class="flex flex-col gap-1 flex-1">
                    <span class="m-0 text-medium text-slate-900 dark:text-slate-50">Try Beta Features</span>
                    <span class="m-0 text-sm text-slate-400">Experience upcoming features before they&apos;re officially released.</span>
                </div>
                <p-toggleswitch
                    [(ngModel)]="checked"
                    inputId="custom"
                    [pt]="{
                        slider: { class: checked ? 'bg-blue-300! rounded-md!' : 'bg-slate-300! dark:bg-slate-600! rounded-md!' },
                        handle: { class: checked ? 'bg-blue-900!' : 'bg-slate-500!' }
                    }"
                />
            </label>
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchCustomizationDemo {
    checked: boolean = true;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div class="flex justify-center">
            <p-toggleswitch [(ngModel)]="checked" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchDisabledDemo {
    checked: boolean = false;
}
```

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div class="flex justify-center">
            <p-toggleswitch [(ngModel)]="checked" [invalid]="!checked" />
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchInvalidDemo {
    checked: boolean = false;
}
```

## preselection-doc

A controlled ToggleSwitch manages the checked state with ngModel and handles the change event manually.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div class="flex justify-center items-center gap-2">
            <p-toggleswitch [(ngModel)]="checked" inputId="mode" />
            <label for="mode">Airplane Mode</label>
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchControlledDemo {
    checked: boolean = true;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { DividerModule } from 'primeng/divider';
import { ToggleSwitchModule } from 'primeng/toggleswitch';

@Component({
    template: `
        <div class="flex flex-col justify-center items-center gap-2 max-w-xs mx-auto w-full">
            <label class="flex items-center justify-between gap-12 select-none">
                <div class="flex-1">
                    <div class="text-color font-bold">Email address</div>
                    <div class="text-muted-color">Users can add email addresses to their account</div>
                </div>
                <p-toggleswitch [(ngModel)]="emailEnabled" />
            </label>
            <p-divider />
            <label class="flex items-center justify-between gap-12 select-none">
                <div class="flex-1">
                    <div class="text-color font-bold">Two-factor authentication</div>
                    <div class="text-muted-color">Require a verification code when signing in</div>
                </div>
                <p-toggleswitch [(ngModel)]="twoFactorEnabled" />
            </label>
        </div>
    `,
    standalone: true,
    imports: [DividerModule, ToggleSwitchModule, FormsModule]
})
export class ToggleSwitchPreviewDemo {
    emailEnabled: boolean = false;

    twoFactorEnabled: boolean = true;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 w-48">
                <div class="flex flex-col items-center gap-2">
                    <p-toggleswitch name="activation" formControlName="activation" [invalid]="isInvalid('activation')" />
                    @if (isInvalid('activation')) {
                        <p-message severity="error" size="small" variant="simple">Activation is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleSwitchModule, ButtonModule, ReactiveFormsModule]
})
export class ToggleSwitchReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            activation: ['', Validators.required]
        });
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        }
    }

    isInvalid(controlName: string) {
        const control = this.exampleForm.get(controlName);

        return control?.invalid && (control.touched || this.formSubmitted);
    }
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, submit, validate } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 w-48">
                <div class="flex flex-col items-center gap-2">
                    <p-toggleswitch name="activation" [formField]="exampleForm.activation" />
                    @if (exampleForm.activation().touched() && exampleForm.activation().invalid()) {
                        @for (error of exampleForm.activation().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleSwitchModule, ButtonModule, FormField]
})
export class ToggleSwitchSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ activation: false });

    exampleForm = form(this.model, (path) => {
        validate(path.activation, ({ value, state }) => {
            if (!state.touched()) return null;

            return value() === true ? null : { kind: 'requiredTrue', message: 'Activation is required.' };
        });
    });

    onSubmit(event: Event) {
        event.preventDefault();
        submit(this.exampleForm, async () => {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        });
    }
}
```

## Template

The handle template is available to display custom content.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex justify-center">
            <p-toggleswitch [(ngModel)]="checked">
                <ng-template #handle let-checked="checked">
                    @if (checked) {
                        <svg data-p-icon="check" class="text-xs!" />
                    } @else {
                        <svg data-p-icon="times" class="text-xs!" />
                    }
                </ng-template>
            </p-toggleswitch>
        </div>
    `,
    standalone: true,
    imports: [ToggleSwitchModule, FormsModule, Check, Times]
})
export class ToggleSwitchTemplateDemo {
    checked: boolean = false;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 w-48">
                <div class="flex flex-col items-center gap-2">
                    <p-toggleswitch #model="ngModel" [(ngModel)]="checked" name="activation" [invalid]="model.invalid && exampleForm.submitted" required />
                    @if (model.invalid && exampleForm.submitted) {
                        <p-message severity="error" size="small" variant="simple">Activation is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleSwitchModule, ButtonModule, FormsModule]
})
export class ToggleSwitchTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    checked: boolean;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Toggle Switch

ToggleSwitch is used to select a boolean value.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ToggleSwitchPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| tabindex | number \| undefined | - | Index of the element in tabbing order. |
| inputId | string \| undefined | - | Identifier of the input element. |
| readonly | boolean | - | When present, it specifies that the component cannot be edited. |
| trueValue | any | - | Value in checked state. |
| falseValue | any | - | Value in unchecked state. |
| ariaLabel | string \| undefined | - | Used to define a string that autocomplete attribute the current element. |
| size | InputSize \| undefined | undefined | Specifies the size of the component. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: ToggleSwitchChangeEvent | Callback to invoke when the on value change. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| handle | TemplateRef<ToggleSwitchHandleTemplateContext> \| undefined | Custom handle template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |
| slider | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the slider's DOM element. |
| handle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the handle's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-toggleswitch | Class name of the root element |
| p-toggleswitch-input | Class name of the input element |
| p-toggleswitch-slider | Class name of the slider element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| toggleswitch.width | --p-toggleswitch-width | Width of root |
| toggleswitch.height | --p-toggleswitch-height | Height of root |
| toggleswitch.border.radius | --p-toggleswitch-border-radius | Border radius of root |
| toggleswitch.gap | --p-toggleswitch-gap | Gap of root |
| toggleswitch.shadow | --p-toggleswitch-shadow | Shadow of root |
| toggleswitch.focus.ring.width | --p-toggleswitch-focus-ring-width | Focus ring width of root |
| toggleswitch.focus.ring.style | --p-toggleswitch-focus-ring-style | Focus ring style of root |
| toggleswitch.focus.ring.color | --p-toggleswitch-focus-ring-color | Focus ring color of root |
| toggleswitch.focus.ring.offset | --p-toggleswitch-focus-ring-offset | Focus ring offset of root |
| toggleswitch.focus.ring.shadow | --p-toggleswitch-focus-ring-shadow | Focus ring shadow of root |
| toggleswitch.border.width | --p-toggleswitch-border-width | Border width of root |
| toggleswitch.border.color | --p-toggleswitch-border-color | Border color of root |
| toggleswitch.hover.border.color | --p-toggleswitch-hover-border-color | Hover border color of root |
| toggleswitch.checked.border.color | --p-toggleswitch-checked-border-color | Checked border color of root |
| toggleswitch.checked.hover.border.color | --p-toggleswitch-checked-hover-border-color | Checked hover border color of root |
| toggleswitch.invalid.border.color | --p-toggleswitch-invalid-border-color | Invalid border color of root |
| toggleswitch.transition.duration | --p-toggleswitch-transition-duration | Transition duration of root |
| toggleswitch.slide.duration | --p-toggleswitch-slide-duration | Slide duration of root |
| toggleswitch.background | --p-toggleswitch-background | Background of root |
| toggleswitch.disabled.background | --p-toggleswitch-disabled-background | Disabled background of root |
| toggleswitch.hover.background | --p-toggleswitch-hover-background | Hover background of root |
| toggleswitch.checked.background | --p-toggleswitch-checked-background | Checked background of root |
| toggleswitch.checked.hover.background | --p-toggleswitch-checked-hover-background | Checked hover background of root |
| toggleswitch.handle.border.radius | --p-toggleswitch-handle-border-radius | Border radius of handle |
| toggleswitch.handle.size | --p-toggleswitch-handle-size | Size of handle |
| toggleswitch.handle.background | --p-toggleswitch-handle-background | Background of handle |
| toggleswitch.handle.disabled.background | --p-toggleswitch-handle-disabled-background | Disabled background of handle |
| toggleswitch.handle.hover.background | --p-toggleswitch-handle-hover-background | Hover background of handle |
| toggleswitch.handle.checked.background | --p-toggleswitch-handle-checked-background | Checked background of handle |
| toggleswitch.handle.checked.hover.background | --p-toggleswitch-handle-checked-hover-background | Checked hover background of handle |
| toggleswitch.handle.color | --p-toggleswitch-handle-color | Color of handle |
| toggleswitch.handle.hover.color | --p-toggleswitch-handle-hover-color | Hover color of handle |
| toggleswitch.handle.checked.color | --p-toggleswitch-handle-checked-color | Checked color of handle |
| toggleswitch.handle.checked.hover.color | --p-toggleswitch-handle-checked-hover-color | Checked hover color of handle |
