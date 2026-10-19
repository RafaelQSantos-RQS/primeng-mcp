---
title: "Angular RadioButton Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/radiobutton.md"
description: "RadioButton is an extension to standard radio button element with theming."
---

# Angular RadioButton Component

RadioButton is an extension to standard radio button element with theming.

## Accessibility

Screen Reader RadioButton component uses a hidden native radio button element internally that is only visible to screen readers. Value to describe the component can either be provided via label tag combined with inputId prop or using ariaLabelledBy , ariaLabel props.

## Basic

Selects a single option from a group of mutually exclusive choices.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="flex flex-col gap-3">
                <div class="flex items-center gap-3">
                    <p-radiobutton inputId="ingredient-strawberry" name="fruit" value="strawberry" [(ngModel)]="ingredient" />
                    <label pLabel for="ingredient-strawberry" class="text-sm">🍓 Strawberry</label>
                </div>
                <div class="flex items-center gap-3">
                    <p-radiobutton inputId="ingredient-banana" name="fruit" value="banana" [(ngModel)]="ingredient" />
                    <label pLabel for="ingredient-banana" class="text-sm">🍌 Banana</label>
                </div>
                <div class="flex items-center gap-3">
                    <p-radiobutton inputId="ingredient-watermelon" name="fruit" value="watermelon" [(ngModel)]="ingredient" />
                    <label pLabel for="ingredient-watermelon" class="text-sm">🍉 Watermelon</label>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, LabelModule, FormsModule]
})
export class RadioButtonBasicDemo {
    ingredient: string | undefined;
}
```

## Card

RadioButtons can be displayed in a card style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { TagModule } from 'primeng/tag';

interface Plan {
    id: string;
    name: string;
    price: string;
    period: string;
    description: string;
    tag: string | null;
}

@Component({
    template: `
        <div class="max-w-xs mx-auto">
            <span class="font-medium">Choose a plan:</span>
            <div class="mt-2 flex flex-col gap-3">
                @for (plan of plans; track plan.id) {
                    <label
                        [for]="'plan-' + plan.id"
                        [class]="'flex-1 flex items-start gap-2 p-3 rounded-lg border hover:bg-surface-100 dark:hover:bg-surface-800 transition-colors cursor-pointer ' + (selectedPlan === plan.id ? 'border-primary' : 'border-surface')"
                    >
                        <div class="flex-1 flex flex-col gap-1">
                            <div class="flex items-center justify-between gap-2">
                                <div class="flex items-center gap-2">
                                    <span class="font-medium leading-none">{{ plan.name }}</span>
                                    @if (plan.tag) {
                                        <p-tag [value]="plan.tag" />
                                    }
                                </div>
                                <div class="text-sm text-color">
                                    <span class="font-semibold">{{ plan.price }}</span>
                                    <span class="text-surface-500">{{ plan.period }}</span>
                                </div>
                            </div>
                            <span class="text-sm text-surface-500">{{ plan.description }}</span>
                        </div>
                        <p-radiobutton [inputId]="'plan-' + plan.id" name="plan" [value]="plan.id" [(ngModel)]="selectedPlan" />
                    </label>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, TagModule, FormsModule]
})
export class RadioButtonCardDemo {
    selectedPlan: string = 'pro';

    plans: Plan[] = [
        { id: 'starter', name: 'Starter', price: '$0', period: '/month', description: 'For solo developers exploring the platform.', tag: null },
        { id: 'pro', name: 'Pro', price: '$29', period: '/month', description: 'For growing teams shipping in production.', tag: 'Popular' },
        { id: 'enterprise', name: 'Enterprise', price: 'Custom', period: '', description: 'For large organizations with custom needs.', tag: null }
    ];
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';

@Component({
    template: `
        <div class="flex justify-center gap-2">
            <p-radiobutton [(ngModel)]="value" [value]="1" [disabled]="true" />
            <p-radiobutton [(ngModel)]="value" [value]="2" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, FormsModule]
})
export class RadioButtonDisabledDemo {
    value: number = 2;
}
```

## Dynamic

RadioButtons can be generated using a list of values.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-wrap gap-4">
                @for (category of categories; track category.key) {
                    <div class="flex items-center gap-2">
                        <p-radiobutton [inputId]="category.key" name="category" [value]="category.key" [(ngModel)]="selectedCategory" />
                        <label pLabel [for]="category.key" class="text-sm">{{ category.name }}</label>
                    </div>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, LabelModule, FormsModule]
})
export class RadioButtonDynamicDemo {
    selectedCategory: string | undefined;

    categories = [
        { name: 'Accounting', key: 'A' },
        { name: 'Marketing', key: 'M' },
        { name: 'Production', key: 'P' },
        { name: 'Research', key: 'R' }
    ];
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';

@Component({
    template: `
        <div class="flex justify-center">
            <p-radiobutton [(ngModel)]="checked" variant="filled" />
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, FormsModule]
})
export class RadioButtonFilledDemo {
    checked: boolean = false;
}
```

## Group

RadioButton is used as a controlled input with value and ngModel properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-wrap gap-4">
                <div class="flex items-center">
                    <p-radiobutton name="pizza" value="Cheese" [(ngModel)]="ingredient" inputId="ingredient1" />
                    <label pLabel for="ingredient1" class="text-sm ml-2">Cheese</label>
                </div>
                <div class="flex items-center">
                    <p-radiobutton name="pizza" value="Mushroom" [(ngModel)]="ingredient" inputId="ingredient2" />
                    <label pLabel for="ingredient2" class="text-sm ml-2">Mushroom</label>
                </div>
                <div class="flex items-center">
                    <p-radiobutton name="pizza" value="Pepper" [(ngModel)]="ingredient" inputId="ingredient3" />
                    <label pLabel for="ingredient3" class="text-sm ml-2">Pepper</label>
                </div>
                <div class="flex items-center">
                    <p-radiobutton name="pizza" value="Onion" [(ngModel)]="ingredient" inputId="ingredient4" />
                    <label pLabel for="ingredient4" class="text-sm ml-2">Onion</label>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, LabelModule, FormsModule]
})
export class RadioButtonGroupDemo {
    ingredient!: string;
}
```

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';

@Component({
    template: `
        <div class="flex justify-center">
            <p-radiobutton [(ngModel)]="value" [invalid]="!value" />
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, FormsModule]
})
export class RadioButtonInvalidDemo {
    value: boolean = false;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';

interface Environment {
    id: string;
    name: string;
    description: string;
    tag: string | null;
    flagClass: string;
}

@Component({
    template: `
        <div class="max-w-xs mx-auto">
            <span class="font-medium">Select a deployment environment:</span>
            <div class="mt-2 flex flex-col gap-3">
                @for (env of environments; track env.id) {
                    <label
                        [for]="'preview-' + env.id"
                        [class]="'flex-1 flex items-start gap-2 p-3 rounded-lg border hover:bg-surface-100 dark:hover:bg-surface-800 transition-colors cursor-pointer ' + (selected === env.id ? 'border-primary' : 'border-surface')"
                    >
                        <div class="flex-1 flex flex-col gap-1">
                            <div class="inline-flex items-center gap-2">
                                <span [class]="'flag w-5! h-auto! aspect-15/10 ' + env.flagClass"></span>
                                <span class="font-medium leading-none">{{ env.name }}</span>
                            </div>
                            <span class="text-sm text-surface-500">{{ env.description }}</span>
                        </div>
                        <p-radiobutton [inputId]="'preview-' + env.id" name="preview-card" [value]="env.id" [(ngModel)]="selected" />
                    </label>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, FormsModule]
})
export class RadioButtonPreviewDemo {
    selected: string | undefined;

    environments: Environment[] = [
        { id: '1', name: 'US East — Virginia', description: 'Low latency, auto-scaling enabled.', tag: 'Faster', flagClass: 'flag-us' },
        { id: '2', name: 'EU West — Frankfurt', description: 'CI/CD pipeline, daily backups.', tag: null, flagClass: 'flag-de' },
        { id: '3', name: 'EU East — Poland', description: 'GDPR compliant, secure.', tag: null, flagClass: 'flag-pl' }
    ];
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { RadioButtonModule } from 'primeng/radiobutton';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4">
                <div class="flex flex-wrap gap-4">
                    @for (category of categories; track category.key) {
                        <div class="flex items-center gap-2">
                            <p-radiobutton formControlName="selectedCategory" name="selectedCategory" [inputId]="'rf_' + category.key" [value]="category" [invalid]="isInvalid('selectedCategory')" />
                            <label pLabel [for]="'rf_' + category.key" class="text-sm"> {{ category.name }} </label>
                        </div>
                    }
                </div>
                @if (isInvalid('selectedCategory')) {
                    <p-message severity="error" size="small" variant="simple"> At least one ingredient must be selected. </p-message>
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RadioButtonModule, ToastModule, ButtonModule, LabelModule, ReactiveFormsModule],
    providers: [MessageService]
})
export class RadioButtonReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    formSubmitted: boolean = false;

    exampleForm: FormGroup;

    categories: any[] = [
        { name: 'Cheese', key: 'C' },
        { name: 'Mushroom', key: 'M' },
        { name: 'Pepper', key: 'P' },
        { name: 'Onion', key: 'O' }
    ];

    constructor() {
        this.exampleForm = this.fb.group({
            selectedCategory: ['', Validators.required]
        });
    }

    isInvalid(controlName: string) {
        const control = this.exampleForm.get(controlName);

        return control?.invalid && this.formSubmitted;
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({
                severity: 'success',
                summary: 'Success',
                detail: 'Form is submitted',
                life: 3000
            });
        }
    }
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { RadioButtonModule } from 'primeng/radiobutton';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { MessageService } from 'primeng/api';

interface Category {
    name: string;
    key: string;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4">
                <div class="flex flex-wrap gap-4">
                    @for (category of categories; track category.key) {
                        <div class="flex items-center gap-2">
                            <p-radiobutton [formField]="exampleForm.selectedCategory" name="selectedCategory" [inputId]="'sf_' + category.key" [value]="category" />
                            <label pLabel [for]="'sf_' + category.key" class="text-sm">{{ category.name }}</label>
                        </div>
                    }
                </div>
                @if (exampleForm.selectedCategory().touched() && exampleForm.selectedCategory().invalid()) {
                    @for (error of exampleForm.selectedCategory().errors(); track error.kind) {
                        <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                    }
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RadioButtonModule, ToastModule, ButtonModule, LabelModule, FormField],
    providers: [MessageService]
})
export class RadioButtonSignalFormsDemo {
    messageService = inject(MessageService);

    categories: Category[] = [
        { name: 'Cheese', key: 'C' },
        { name: 'Mushroom', key: 'M' },
        { name: 'Pepper', key: 'P' },
        { name: 'Onion', key: 'O' }
    ];

    model = signal<{ selectedCategory: Category | null }>({ selectedCategory: null });

    exampleForm = form(this.model, (path) => {
        required(path.selectedCategory, { when: ({ state }) => state.touched(), message: 'Please select a category.' });
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

RadioButton provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RadioButtonModule } from 'primeng/radiobutton';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-wrap gap-4">
                <div class="flex items-center gap-2">
                    <p-radiobutton [(ngModel)]="size" inputId="size_small" name="size" value="Small" size="small" />
                    <label pLabel for="size_small" class="text-sm">Small</label>
                </div>
                <div class="flex items-center gap-2">
                    <p-radiobutton [(ngModel)]="size" inputId="size_normal" name="size" value="Normal" />
                    <label pLabel for="size_normal">Normal</label>
                </div>
                <div class="flex items-center gap-2">
                    <p-radiobutton [(ngModel)]="size" inputId="size_large" name="size" value="Large" size="large" />
                    <label pLabel for="size_large" class="text-lg!">Large</label>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [RadioButtonModule, LabelModule, FormsModule]
})
export class RadioButtonSizesDemo {
    size: string = 'Normal';
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { RadioButtonModule } from 'primeng/radiobutton';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4">
                <div class="flex flex-wrap gap-4">
                    @for (category of categories; track category.name) {
                        <div class="flex items-center gap-2">
                            <p-radiobutton [(ngModel)]="ingredient" [inputId]="'td_' + category.key" [value]="category" [invalid]="isInvalid(exampleForm)" name="ingredient" />
                            <label pLabel [for]="'td_' + category.key" class="text-sm"> {{ category.name }} </label>
                        </div>
                    }
                </div>
                @if (isInvalid(exampleForm)) {
                    <p-message severity="error" size="small" variant="simple"> At least one ingredient must be selected. </p-message>
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RadioButtonModule, ToastModule, ButtonModule, LabelModule, FormsModule],
    providers: [MessageService]
})
export class RadioButtonTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    formSubmitted: boolean = false;

    ingredient!: any;

    categories: any[] = [
        { name: 'Cheese', key: 'C' },
        { name: 'Mushroom', key: 'M' },
        { name: 'Pepper', key: 'P' },
        { name: 'Onion', key: 'O' }
    ];

    isInvalid(form: NgForm) {
        return !this.ingredient && form.submitted;
    }

    onSubmit(form: NgForm) {
        if (!this.isInvalid(form)) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Radio Button

RadioButton is an extension to standard radio button element with theming.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, RadioButtonPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| value | any | - | Value of the radiobutton. |
| tabindex | number \| undefined | - | Index of the element in tabbing order. |
| inputId | string \| undefined | - | Identifier of the focus input to match a label defined for the component. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| ariaLabel | string \| undefined | - | Used to define a string that labels the input element. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| binary | boolean | - | Allows to select a boolean value. |
| variant | InputVariant \| undefined | undefined | Specifies the input variant of the component. |
| size | InputSize \| undefined | undefined | Specifies the size of the component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onClick | event: RadioButtonClickEvent | Callback to invoke on radio button click. |
| onFocus | event: Event | Callback to invoke when the receives focus. |
| onBlur | event: Event | Callback to invoke when the loses focus. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| focus |  | void | Applies focus to input field. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |
| box | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the box's DOM element. |
| icon | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-radiobutton | Class name of the root element |
| p-radiobutton-box | Class name of the box element |
| p-radiobutton-input | Class name of the input element |
| p-radiobutton-icon | Class name of the icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| radiobutton.width | --p-radiobutton-width | Width of root |
| radiobutton.height | --p-radiobutton-height | Height of root |
| radiobutton.background | --p-radiobutton-background | Background of root |
| radiobutton.checked.background | --p-radiobutton-checked-background | Checked background of root |
| radiobutton.checked.hover.background | --p-radiobutton-checked-hover-background | Checked hover background of root |
| radiobutton.disabled.background | --p-radiobutton-disabled-background | Disabled background of root |
| radiobutton.filled.background | --p-radiobutton-filled-background | Filled background of root |
| radiobutton.border.color | --p-radiobutton-border-color | Border color of root |
| radiobutton.hover.border.color | --p-radiobutton-hover-border-color | Hover border color of root |
| radiobutton.focus.border.color | --p-radiobutton-focus-border-color | Focus border color of root |
| radiobutton.checked.border.color | --p-radiobutton-checked-border-color | Checked border color of root |
| radiobutton.checked.hover.border.color | --p-radiobutton-checked-hover-border-color | Checked hover border color of root |
| radiobutton.checked.focus.border.color | --p-radiobutton-checked-focus-border-color | Checked focus border color of root |
| radiobutton.checked.disabled.border.color | --p-radiobutton-checked-disabled-border-color | Checked disabled border color of root |
| radiobutton.invalid.border.color | --p-radiobutton-invalid-border-color | Invalid border color of root |
| radiobutton.shadow | --p-radiobutton-shadow | Shadow of root |
| radiobutton.focus.ring.width | --p-radiobutton-focus-ring-width | Focus ring width of root |
| radiobutton.focus.ring.style | --p-radiobutton-focus-ring-style | Focus ring style of root |
| radiobutton.focus.ring.color | --p-radiobutton-focus-ring-color | Focus ring color of root |
| radiobutton.focus.ring.offset | --p-radiobutton-focus-ring-offset | Focus ring offset of root |
| radiobutton.focus.ring.shadow | --p-radiobutton-focus-ring-shadow | Focus ring shadow of root |
| radiobutton.transition.duration | --p-radiobutton-transition-duration | Transition duration of root |
| radiobutton.sm.width | --p-radiobutton-sm-width | Sm width of root |
| radiobutton.sm.height | --p-radiobutton-sm-height | Sm height of root |
| radiobutton.lg.width | --p-radiobutton-lg-width | Lg width of root |
| radiobutton.lg.height | --p-radiobutton-lg-height | Lg height of root |
| radiobutton.icon.size | --p-radiobutton-icon-size | Size of icon |
| radiobutton.icon.checked.color | --p-radiobutton-icon-checked-color | Checked color of icon |
| radiobutton.icon.checked.hover.color | --p-radiobutton-icon-checked-hover-color | Checked hover color of icon |
| radiobutton.icon.disabled.color | --p-radiobutton-icon-disabled-color | Disabled color of icon |
| radiobutton.icon.sm.size | --p-radiobutton-icon-sm-size | Sm size of icon |
| radiobutton.icon.lg.size | --p-radiobutton-icon-lg-size | Lg size of icon |
