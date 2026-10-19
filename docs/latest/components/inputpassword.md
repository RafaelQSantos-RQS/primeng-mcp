---
title: "Angular InputPassword Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inputpassword.md"
description: "InputPassword is an enhanced input for password entry with strength metering, mask toggling, and controlled or uncontrolled usage."
---

# Angular InputPassword Component

InputPassword is an enhanced input for password entry with strength metering, mask toggling, and controlled or uncontrolled usage.

## Accessibility

Screen Reader InputPassword is applied to a native input element so it implicitly includes any passed prop. Value to describe the component can either be provided via label tag combined with id prop or using aria-labelledby , aria-label props.

## Basic

InputPassword is applied to an input element with the pInputPassword directive.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputPassword [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule]
})
export class InputPasswordBasicDemo {
    value: string = '';
}
```

## Clear Icon

Use a custom clear action to reset the password input.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iconfield>
                <input pInputPassword [(ngModel)]="value" class="w-56" />
                @if (value) {
                    <p-inputicon (click)="value = ''" style="cursor: pointer">
                        <svg data-p-icon="times"></svg>
                    </p-inputicon>
                }
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputPasswordModule, FormsModule, Times]
})
export class InputPasswordCleariconDemo {
    value: string = '';
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputPassword disabled placeholder="Disabled" />
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule]
})
export class InputPasswordDisabledDemo {}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputPassword variant="filled" [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule]
})
export class InputPasswordFilledDemo {
    value: string = '';
}
```

## Float Label

FloatLabel visually integrates a label with its form element. Visit FloatLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FloatLabelModule } from 'primeng/floatlabel';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex flex-wrap justify-center items-end gap-4">
            <p-floatlabel>
                <input pInputPassword id="over_label" [(ngModel)]="value1" autocomplete="off" />
                <label for="over_label">Password</label>
            </p-floatlabel>
            <p-floatlabel variant="in">
                <input pInputPassword id="in_label" [(ngModel)]="value2" autocomplete="off" />
                <label for="in_label">Password</label>
            </p-floatlabel>
            <p-floatlabel variant="on">
                <input pInputPassword id="on_label" [(ngModel)]="value3" autocomplete="off" />
                <label for="on_label">Password</label>
            </p-floatlabel>
        </div>
    `,
    standalone: true,
    imports: [FloatLabelModule, InputPasswordModule, FormsModule]
})
export class InputPasswordFloatLabelDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## Fluid

Fluid spans the full width of the container.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <input pInputPassword [fluid]="true" [(ngModel)]="value" />
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule]
})
export class InputPasswordFluidDemo {
    value: string = '';
}
```

## Ifta Label

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iftalabel>
                <input pInputPassword id="password" variant="filled" [(ngModel)]="value" autocomplete="off" />
                <label for="password">Password</label>
            </p-iftalabel>
        </div>
    `,
    standalone: true,
    imports: [IftaLabelModule, InputPasswordModule, FormsModule]
})
export class InputPasswordIftaLabelDemo {
    value: string | undefined;
}
```

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <input pInputPassword [(ngModel)]="value1" [invalid]="!value1" placeholder="Password" />
            <input pInputPassword variant="filled" [(ngModel)]="value2" [invalid]="!value2" placeholder="Password" />
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule]
})
export class InputPasswordInvalidDemo {
    value1: string = '';

    value2: string = '';
}
```

## Popover

Combine a visibility toggle, strength meter, and requirements checklist into a fully custom password creation with Popover component.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { Popover, PopoverModule } from 'primeng/popover';
import { ProgressBarModule } from 'primeng/progressbar';
import { TagModule } from 'primeng/tag';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Eye } from '@primeicons/angular/eye';
import { EyeSlash } from '@primeicons/angular/eye-slash';
import { Shield } from '@primeicons/angular/shield';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col gap-2">
                <p-iconfield>
                    <input pInputPassword [(ngModel)]="value" [(mask)]="mask" placeholder="Create a password" (focus)="op.show($event)" (blur)="op.hide()" />
                    <p-inputicon (click)="mask = !mask" style="cursor: pointer">
                        @if (mask) {
                            <svg data-p-icon="eye"></svg>
                        } @else {
                            <svg data-p-icon="eye-slash"></svg>
                        }
                    </p-inputicon>
                </p-iconfield>
                <p-popover #op>
                    <div class="flex flex-col gap-3 w-72">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <svg data-p-icon="shield" style="width: 1.25rem; height: 1.25rem"></svg>
                                <span class="font-semibold text-sm">Password Strength</span>
                            </div>
                            <p-tag [severity]="severity" [value]="label || '—'" [style.visibility]="label ? 'visible' : 'hidden'" />
                        </div>
                        <p-progressbar [value]="score" [showValue]="false" [style]="{ height: '6px' }" [color]="color" />
                        <ul class="flex flex-col gap-2 list-none m-0 p-0">
                            @for (rule of rules; track rule.id) {
                                <li class="flex items-center gap-2 text-xs">
                                    @if (rule.test(value)) {
                                        <svg data-p-icon="check" class="text-green-500"></svg>
                                    } @else {
                                        <svg data-p-icon="times" class="text-red-400"></svg>
                                    }
                                    <span [class]="rule.test(value) ? 'text-surface-500' : 'text-surface-700'">{{ rule.label }}</span>
                                </li>
                            }
                        </ul>
                    </div>
                </p-popover>
            </div>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, PopoverModule, ProgressBarModule, TagModule, InputPasswordModule, FormsModule, Eye, EyeSlash, Shield, Check, Times]
})
export class InputPasswordPopoverDemo {
    value: string = '';

    mask: boolean = true;

    rules = [
        { id: 'length', label: 'At least 12 characters long', test: (v: string) => v.length >= 12, weight: 20 },
        { id: 'uppercase', label: 'Contains uppercase letter', test: (v: string) => /[A-Z]/.test(v), weight: 20 },
        { id: 'lowercase', label: 'Contains lowercase letter', test: (v: string) => /[a-z]/.test(v), weight: 20 },
        { id: 'number', label: 'Contains number', test: (v: string) => /[0-9]/.test(v), weight: 20 },
        { id: 'special', label: 'Contains special character (!@#$...)', test: (v: string) => /[^a-zA-Z0-9]/.test(v), weight: 20 }
    ];

    get score(): number {
        if (!this.value) return 0;

        return this.rules.reduce((acc, rule) => acc + (rule.test(this.value) ? rule.weight : 0), 0);
    }

    get severity(): 'danger' | 'warn' | 'info' | 'success' {
        if (this.score <= 20) return 'danger';

        if (this.score <= 40) return 'warn';

        if (this.score <= 60) return 'info';

        return 'success';
    }

    get color(): string {
        switch (this.severity) {
            case 'danger':
                return 'var(--p-red-500)';
            case 'warn':
                return 'var(--p-amber-500)';
            case 'info':
                return 'var(--p-blue-500)';
            case 'success':
                return 'var(--p-green-500)';
        }
    }

    get label(): string {
        if (this.score === 0) return '';

        if (this.score <= 20) return 'Too Weak';

        if (this.score <= 40) return 'Weak';

        if (this.score <= 60) return 'Fair';

        if (this.score <= 80) return 'Strong';

        return 'Very Strong';
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ChipModule } from 'primeng/chip';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Lock } from '@primeicons/angular/lock';
import { Eye } from '@primeicons/angular/eye';
import { EyeSlash } from '@primeicons/angular/eye-slash';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="max-w-2xs w-full">
                <div class="text-sm font-medium text-color mb-2">Create a password</div>
                <p-iconfield>
                    <p-inputicon>
                        <svg data-p-icon="lock"></svg>
                    </p-inputicon>
                    <input pInputPassword [(ngModel)]="value" [(mask)]="mask" [fluid]="true" />
                    <p-inputicon (click)="mask = !mask" style="cursor: pointer">
                        @if (mask) {
                            <svg data-p-icon="eye" [size]="16"></svg>
                        } @else {
                            <svg data-p-icon="eye-slash" [size]="16"></svg>
                        }
                    </p-inputicon>
                </p-iconfield>
                <div class="mt-4 flex items-center flex-wrap gap-1.5">
                    @for (rule of rules; track rule.label) {
                        <p-chip [class]="'py-0.75! pl-1.5! pr-2.5! text-xs! ' + (rule.test(value) ? 'text-green-600! dark:text-green-400!' : 'text-surface-500! dark:text-surface-400!')">
                            <span
                                [class]="
                                    'size-4 -ml-0.75 inline-flex items-center justify-center rounded-full ' +
                                    (rule.test(value) ? 'bg-green-600 text-surface-0 dark:bg-green-400 dark:text-surface-900' : 'bg-surface-200 dark:bg-surface-700 text-surface-500 dark:text-surface-400')
                                "
                            >
                                @if (rule.test(value)) {
                                    <svg data-p-icon="check" [size]="12"></svg>
                                } @else {
                                    <svg data-p-icon="times" [size]="12"></svg>
                                }
                            </span>
                            {{ rule.label }}
                        </p-chip>
                    }
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ChipModule, IconFieldModule, InputIconModule, InputPasswordModule, FormsModule, Lock, Eye, EyeSlash, Check, Times]
})
export class InputPasswordPreviewDemo {
    value: string = '1234';

    mask: boolean = true;

    rules = [
        { label: '8+ characters', test: (v: string) => v.length >= 8 },
        { label: 'Number', test: (v: string) => /\d/.test(v) },
        { label: 'Uppercase letter', test: (v: string) => /[A-Z]/.test(v) },
        { label: 'Special character', test: (v: string) => /[^a-zA-Z0-9]/.test(v) }
    ];
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputPasswordModule } from 'primeng/inputpassword';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <input pInputPassword formControlName="value" [invalid]="isInvalid('value')" autocomplete="off" placeholder="Password" />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">Password is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ButtonModule, InputPasswordModule, ReactiveFormsModule],
    providers: [MessageService]
})
export class InputPasswordReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            value: ['', Validators.required]
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

## Requirements

Password requirements checklist with real-time validation feedback.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';
import { CheckCircle } from '@primeicons/angular/check-circle';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col gap-2 w-64">
                <input pInputPassword [(ngModel)]="value" placeholder="Enter password" />
                <ul class="flex flex-col gap-2 list-none ms-1 my-1 p-0">
                    @for (req of requirements; track req.id) {
                        <li class="flex items-center gap-2 text-sm transition-all duration-300">
                            <svg data-p-icon="check-circle" [class]="'transition-all duration-300 ease-out ' + (req.test(value) ? 'text-green-500 scale-110 opacity-100' : 'text-surface-400 scale-90 opacity-70')"></svg>
                            <span
                                [class]="
                                    'transition-all duration-300 ease-out ' + (req.test(value) ? 'text-green-700 dark:text-green-400 line-through decoration-2 decoration-green-500/70' : 'text-surface-400 dark:text-surface-300 dark:opacity-70')
                                "
                            >
                                {{ req.label }}
                            </span>
                        </li>
                    }
                </ul>
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule, CheckCircle]
})
export class InputPasswordRequirementsDemo {
    value: string = '';

    requirements = [
        { id: 'minLength', label: 'At least 12 characters', test: (v: string) => v.length >= 12 },
        { id: 'uppercase', label: 'Contains uppercase letter', test: (v: string) => /[A-Z]/.test(v) },
        { id: 'lowercase', label: 'Contains lowercase letter', test: (v: string) => /[a-z]/.test(v) },
        { id: 'number', label: 'Contains number', test: (v: string) => /[0-9]/.test(v) },
        { id: 'symbol', label: 'Contains special character', test: (v: string) => /[^a-zA-Z0-9]/.test(v) }
    ];
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputPasswordModule } from 'primeng/inputpassword';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <input pInputPassword [formField]="exampleForm.value" autocomplete="off" placeholder="Password" />
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
    imports: [MessageModule, ToastModule, ButtonModule, InputPasswordModule, FormField]
})
export class InputPasswordSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ value: '' });

    exampleForm = form(this.model, (path) => {
        required(path.value, { when: ({ state }) => state.touched(), message: 'Password is required.' });
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

InputPassword provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <input pInputPassword pSize="small" [(ngModel)]="value1" placeholder="Small" />
            <input pInputPassword [(ngModel)]="value2" placeholder="Normal" />
            <input pInputPassword pSize="large" [(ngModel)]="value3" placeholder="Large" />
        </div>
    `,
    standalone: true,
    imports: [InputPasswordModule, FormsModule]
})
export class InputPasswordSizesDemo {
    value1: string = '';

    value2: string = '';

    value3: string = '';
}
```

## Strength Meter

Visualize the overall password strength with an animated progress bar and a severity-based label that adapts as the password improves.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ProgressBarModule } from 'primeng/progressbar';
import { TagModule } from 'primeng/tag';
import { InputPasswordModule } from 'primeng/inputpassword';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col gap-2 w-64">
                <input pInputPassword [(ngModel)]="value" placeholder="Enter password" />
                <div class="flex flex-col gap-2" [style.visibility]="strengthInfo ? 'visible' : 'hidden'">
                    <p-progressbar [value]="strengthInfo?.percent ?? 0" [showValue]="false" [style]="{ height: '6px' }" [color]="strengthInfo?.color" />
                    <div class="flex justify-end">
                        <p-tag [severity]="strengthInfo?.severity ?? 'info'" [value]="strengthInfo?.label ?? '—'" />
                    </div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ProgressBarModule, TagModule, InputPasswordModule, FormsModule]
})
export class InputPasswordStrengthDemo {
    value: string = '';

    get strengthInfo() {
        const level = this.getStrength(this.value);

        return level ? this.strengthMap[level] : null;
    }

    getStrength(value: string): string | null {
        if (!value) return null;

        let score = 0;

        if (value.length >= 8) score++;

        if (value.length >= 12) score++;

        if (/[A-Z]/.test(value) && /[a-z]/.test(value)) score++;

        if (/[0-9]/.test(value)) score++;

        if (/[^a-zA-Z0-9]/.test(value)) score++;

        if (score <= 1) return 'weak';

        if (score <= 2) return 'medium';

        if (score <= 3) return 'strong';

        return 'very-strong';
    }
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputPasswordModule } from 'primeng/inputpassword';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <input pInputPassword #model="ngModel" [(ngModel)]="value" [invalid]="model.invalid && (model.touched || exampleForm.submitted)" name="password" autocomplete="off" placeholder="Password" required />
                    @if (model.invalid && (model.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Password is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ButtonModule, InputPasswordModule, FormsModule],
    providers: [MessageService]
})
export class InputPasswordTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: any;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Toggle Mask

Adding a toggle icon to show or hide the password, allowing users to verify their input.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Eye } from '@primeicons/angular/eye';
import { EyeSlash } from '@primeicons/angular/eye-slash';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iconfield>
                <input pInputPassword [(ngModel)]="value" [(mask)]="mask" />
                <p-inputicon (click)="mask = !mask" style="cursor: pointer">
                    @if (mask) {
                        <svg data-p-icon="eye"></svg>
                    } @else {
                        <svg data-p-icon="eye-slash"></svg>
                    }
                </p-inputicon>
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputPasswordModule, FormsModule, Eye, EyeSlash]
})
export class InputPasswordTogglemaskDemo {
    value: string = '';

    mask: boolean = true;
}
```

## Input Password

InputPassword directive is an extension to standard input element with mask toggling.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InputPasswordPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| mask | boolean | true | Whether the password is masked. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inputpassword | The class of root element |
