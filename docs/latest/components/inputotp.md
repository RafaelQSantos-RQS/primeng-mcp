---
title: "Angular Otp Input Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inputotp.md"
description: "Input Otp is used to enter one time passwords."
---

# Angular Otp Input Component

Input Otp is used to enter one time passwords.

## Accessibility

Screen Reader Input OTP uses a set of InputText components, refer to the InputText component for more information about the screen reader support.

## Basic

A one-time password input split into individual digit fields.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpBasicDemo {
    value: any;
}
```

## Controlled

InputOtp can be used as a controlled component with ngModel .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex flex-col items-center gap-6">
            <p-inputotp [(ngModel)]="value" />
            <div class="flex items-center gap-4">
                <span class="text-sm text-muted-color">
                    Value: <strong class="text-color">{{ value || '' }}</strong>
                </span>
                <button pButton severity="secondary" size="small" (click)="value = ''">Reset</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, ButtonModule, FormsModule]
})
export class InputOtpControlledDemo {
    value: any = '';
}
```

## Disabled

When disabled is present, the component becomes non-interactive.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpDisabledDemo {
    value: any;
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value" variant="filled" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpFilledDemo {
    value: any;
}
```

## Integer Only

When integerOnly is present, only integers can be accepted as input.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value" [integerOnly]="true" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpIntegerOnlyDemo {
    value: any;
}
```

## Mask

Enable the mask option to hide the values in the input fields.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value" [mask]="true" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpMaskDemo {
    value: any;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex flex-col gap-10 justify-between h-full max-w-xs mx-auto">
            <div>
                <div class="font-bold text-color text-lg">Check your email for OTP</div>
                <div class="text-surface-500 mt-1">Please enter the 6-digit code sent to your email address to reset your password.</div>
            </div>
            <p-inputotp [(ngModel)]="value" [length]="6" [integerOnly]="true" class="w-full *:flex-1 *:aspect-square **:text-2xl! **:font-bold! my-auto gap-3" />
            <div>
                <div class="text-sm text-muted-color mb-2">
                    Didn't receive code?
                    <button pButton variant="text" class="text-sm">Resend</button>
                </div>
                <button pButton class="w-full">Change password</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, ButtonModule, FormsModule]
})
export class InputOtpPreviewDemo {
    value: any = '482';
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <p-inputotp formControlName="value" [invalid]="isInvalid('value')" />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">Passcode is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, MessageModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class InputOtpReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            value: ['', [Validators.required, Validators.minLength(4)]]
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

## Sample

A sample UI implementation with templating and additional elements.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col items-center">
                <div class="font-bold text-xl mb-2">Authenticate Your Account</div>
                <p class="text-sm text-muted-color block mb-8">Please enter the code sent to your phone.</p>
                <p-inputotp [(ngModel)]="value" [length]="6">
                    <ng-template #input let-token let-events="events" let-index="index">
                        <input type="text" [maxLength]="1" (input)="events.input($event)" (keydown)="events.keydown($event)" [attr.value]="token" class="custom-otp-input" />
                        @if (index === 3) {
                            <div class="px-4">
                                <i class="pi pi-minus"></i>
                            </div>
                        }
                    </ng-template>
                </p-inputotp>
                <div class="flex justify-between mt-8 self-stretch">
                    <button type="button" pButton [link]="true" class="p-0">Resend Code</button>
                    <button type="button" pButton>Submit Code</button>
                </div>
            </div>
        </div>
    `,
    styles: [
        `
            .p-inputotp {
                gap: 0;
            }

            .custom-otp-input {
                width: 48px;
                height: 48px;
                font-size: 24px;
                appearance: none;
                text-align: center;
                transition: all 0.2s;
                border-radius: 0;
                border: 1px solid var(--p-inputtext-border-color);
                background: transparent;
                outline-offset: -2px;
                outline-color: transparent;
                border-right: 0 none;
                transition: outline-color 0.3s;
                color: var(--p-inputtext-color);
            }

            .custom-otp-input:focus {
                outline: 2px solid var(--p-focus-ring-color);
            }

            .custom-otp-input:first-child,
            .custom-otp-input:nth-child(5) {
                border-top-left-radius: 12px;
                border-bottom-left-radius: 12px;
            }

            .custom-otp-input:nth-child(3),
            .custom-otp-input:last-child {
                border-top-right-radius: 12px;
                border-bottom-right-radius: 12px;
                border-right-width: 1px;
                border-right-style: solid;
                border-color: var(--p-inputtext-border-color);
            }
        `
    ],
    standalone: true,
    imports: [InputOtpModule, ButtonModule, FormsModule]
})
export class InputOtpSampleDemo {
    value: any;
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit, validate } from '@angular/forms/signals';
import { InputOtpModule } from 'primeng/inputotp';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <p-inputotp [formField]="exampleForm.value" />
                    @if (exampleForm.value().touched() && exampleForm.value().invalid()) {
                        @for (error of exampleForm.value().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, MessageModule, ToastModule, ButtonModule, FormField]
})
export class InputOtpSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ value: '' });

    exampleForm = form(this.model, (path) => {
        required(path.value, { when: ({ state }) => state.touched(), message: 'Passcode is required.' });
        validate(path.value, ({ value, state }) => {
            if (!state.touched() || !value() || value().length >= 4) return undefined;

            return { kind: 'minLength', message: 'Please enter all 4 digits.' };
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

## Sizes

InputOtp provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <p-inputotp [(ngModel)]="value1" size="small" />
            <p-inputotp [(ngModel)]="value2" />
            <p-inputotp [(ngModel)]="value3" size="large" />
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpSizesDemo {
    value1: any;

    value2: any;

    value3: any;
}
```

## Template

Define a template with your own UI elements with bindings to the provided events and attributes to replace the default design.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputotp [(ngModel)]="value">
                <ng-template #input let-token let-events="events">
                    <input class="custom-otp-input" (keydown)="events.keydown($event)" (input)="events.input($event)" type="text" [attr.value]="token" [maxLength]="1" />
                </ng-template>
            </p-inputotp>
        </div>
    `,
    styles: [
        `
            .custom-otp-input {
                width: 40px;
                font-size: 36px;
                border: 0 none;
                appearance: none;
                text-align: center;
                transition: all 0.2s;
                background: transparent;
                border-bottom: 2px solid var(--p-inputtext-border-color);
                border-radius: 0px;
            }

            .custom-otp-input:focus {
                outline: 0 none;
                border-bottom-color: var(--p-primary-color);
            }
        `
    ],
    standalone: true,
    imports: [InputOtpModule, FormsModule]
})
export class InputOtpTemplateDemo {
    value: any;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputOtpModule } from 'primeng/inputotp';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <p-inputotp #otpModel="ngModel" [(ngModel)]="value" [invalid]="otpModel.invalid && (otpModel.touched || exampleForm.submitted)" name="value" required [minlength]="4" />
                    @if (otpModel.invalid && (otpModel.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Passcode is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [InputOtpModule, MessageModule, ToastModule, ButtonModule, FormsModule]
})
export class InputOtpTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: any;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Input Otp

Input Otp is used to enter one time passwords.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InputOtpPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| readonly | boolean | - | When present, it specifies that an input field is read-only. |
| tabindex | number \| null | - | Index of the element in tabbing order. |
| length | number | - | Number of characters to initiate. |
| styleClass | string \| undefined | - | Style class of the input element. |
| mask | boolean | - | Mask pattern. |
| integerOnly | boolean | - | When present, it specifies that an input field is integer-only. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| variant | InputVariant \| undefined | undefined | Specifies the input variant of the component. |
| size | InputSize \| undefined | undefined | Specifies the size of the component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: InputOtpChangeEvent | Callback to invoke on value change. |
| onFocus | event: Event | Callback to invoke when the component receives focus. |
| onBlur | event: Event | Callback to invoke when the component loses focus. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| input | TemplateRef<InputOtpInputTemplateContext> \| undefined | Custom input template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| pcInputText | InputTextPassThrough | Used to pass attributes to the InputText component. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inputotp | Class name of the root element |
| p-inputotp-input | Class name of the input element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| inputotp.gap | --p-inputotp-gap | Gap of root |
| inputotp.input.width | --p-inputotp-input-width | Width of input |
| inputotp.input.sm.width | --p-inputotp-input-sm-width | Width of input in small screens |
| inputotp.input.lg.width | --p-inputotp-input-lg-width | Width of input in large screens |
