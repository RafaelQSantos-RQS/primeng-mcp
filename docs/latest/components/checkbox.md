---
title: "Angular Checkbox Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/checkbox.md"
description: "Checkbox is an extension to standard checkbox element with theming."
---

# Angular Checkbox Component

Checkbox is an extension to standard checkbox element with theming.

## Accessibility

Screen Reader Checkbox component uses a hidden native checkbox element internally that is only visible to screen readers. Value to describe the component can either be provided via label tag combined with inputId prop or using ariaLabelledBy , ariaLabel props.

## Basic

Binary checkbox is used as a controlled input with ngModel and binary properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';

@Component({
    template: `
        <div class="flex justify-center gap-4">
            <p-checkbox [(ngModel)]="checked" [binary]="true" />
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, FormsModule]
})
export class CheckboxBasicDemo {
    checked: boolean = false;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';

@Component({
    template: `
        <div class="flex items-center justify-center gap-4">
            <p-checkbox [(ngModel)]="checked1" [binary]="true" [disabled]="true" />
            <p-checkbox [(ngModel)]="checked2" [binary]="true" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, FormsModule]
})
export class CheckboxDisabledDemo {
    checked1: boolean = false;

    checked2: boolean = true;
}
```

## Dynamic

Checkboxes can be generated using a list of values.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col gap-4">
                @for (category of categories; track category.key) {
                    <div class="flex items-center gap-2">
                        <p-checkbox [inputId]="category.key" name="group" [value]="category" [(ngModel)]="selectedCategories" />
                        <label pLabel [for]="category.key" class="text-sm">{{ category.name }}</label>
                    </div>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxDynamicDemo implements OnInit {
    selectedCategories: any[] = [];

    categories: any[] = [
        { name: 'Accounting', key: 'A' },
        { name: 'Marketing', key: 'M' },
        { name: 'Production', key: 'P' },
        { name: 'Research', key: 'R' }
    ];

    ngOnInit() {
        this.selectedCategories = [this.categories[1]];
    }
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="checked" [binary]="true" variant="filled" inputId="filled-checkbox" />
                <label pLabel for="filled-checkbox" class="text-sm">Filled</label>
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxFilledDemo {
    checked: boolean = false;
}
```

## Indeterminate

The indeterminate state indicates that a checkbox is neither "on" or "off".

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex items-center gap-2 justify-center">
            <p-checkbox [(ngModel)]="checked" [binary]="true" [indeterminate]="true" inputId="indeterminate-checkbox" />
            <label pLabel for="indeterminate-checkbox" class="text-sm">Email Notifications</label>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxIndeterminateDemo {
    checked: boolean = false;
}
```

## Indicator

Use the icon template to customize the visual indicator inside the checkbox.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';
import { Check } from '@primeicons/angular/check';
import { Minus } from '@primeicons/angular/minus';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="checked" [binary]="true" [indeterminate]="true" inputId="checkbox-indicator" (onChange)="onCheck()">
                    <ng-template #icon let-checked="checked" let-iconClass="class">
                        @if (checked) {
                            <svg data-p-icon="check" [class]="iconClass" />
                        } @else if (isIndeterminate) {
                            <svg data-p-icon="minus" [class]="iconClass" />
                        } @else {
                            <svg data-p-icon="times" [class]="iconClass" />
                        }
                    </ng-template>
                </p-checkbox>
                <label pLabel for="checkbox-indicator" class="text-sm">Select all notifications</label>
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule, Check, Minus, Times]
})
export class CheckboxIndicatorDemo {
    checked: boolean = false;

    isIndeterminate: boolean = true;

    onCheck() {
        this.isIndeterminate = false;
    }
}
```

## Invalid

The invalid state is applied using the invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="checked" [binary]="true" [invalid]="!checked" inputId="invalid-checkbox" />
                <label pLabel for="invalid-checkbox" [class]="'transition-colors ' + (!checked ? 'text-red-500! dark:text-red-400!' : '')">Invalid</label>
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxInvalidDemo {
    checked: boolean = false;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="checked" [binary]="true" inputId="preview-checkbox" />
                <label pLabel for="preview-checkbox"> I accept the <a class="font-semibold">Terms of Service</a> </label>
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxPreviewDemo {
    checked: boolean = false;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { MessageModule } from 'primeng/message';
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
                    @for (item of formKeys; track item) {
                        <div class="flex items-center gap-2">
                            <p-checkbox [formControlName]="item" [binary]="true" [inputId]="'rf_' + item" [invalid]="isInvalid(item)" />
                            <label pLabel [for]="'rf_' + item" class="text-sm"> {{ item | titlecase }} </label>
                        </div>
                    }
                </div>
                @if (hasAnyInvalid()) {
                    <p-message severity="error" size="small" variant="simple"> At least one ingredient must be selected. </p-message>
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, MessageModule, ToastModule, ButtonModule, LabelModule, ReactiveFormsModule]
})
export class CheckboxReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    formSubmitted: boolean = false;

    exampleForm: FormGroup;

    constructor() {
        this.exampleForm = this.fb.group(
            {
                cheese: [false],
                mushroom: [false],
                pepper: [false],
                onion: [false]
            },
            { validators: this.atLeastOneSelectedValidator }
        );
    }

    get formKeys(): string[] {
        return Object.keys(this.exampleForm.controls);
    }

    atLeastOneSelectedValidator(group: FormGroup) {
        [key: string]: any
    }

    hasAnyInvalid(): boolean {
        return this.formSubmitted && this.exampleForm.hasError('atLeastOneRequired');
    }

    isInvalid(controlName: string): boolean {
        const control = this.exampleForm.get(controlName);

        return this.formSubmitted && this.exampleForm.hasError('atLeastOneRequired') && control?.value === false;
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
import { form, FormField, submit, validate } from '@angular/forms/signals';
import { CheckboxModule } from 'primeng/checkbox';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { MessageService } from 'primeng/api';

interface Toppings {
    cheese: boolean;
    mushroom: boolean;
    pepper: boolean;
    onion: boolean;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4">
                <div class="flex flex-wrap gap-4">
                    @for (key of formKeys; track key) {
                        <div class="flex items-center gap-2">
                            <p-checkbox [formField]="exampleForm[key]" [binary]="true" [inputId]="'sf_' + key" />
                            <label pLabel [for]="'sf_' + key" class="text-sm">{{ key | titlecase }}</label>
                        </div>
                    }
                </div>
                @if (exampleForm().touched() && exampleForm().invalid()) {
                    @for (error of exampleForm().errors(); track error.kind) {
                        <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                    }
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, MessageModule, ToastModule, ButtonModule, LabelModule, FormField]
})
export class CheckboxSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal<Toppings>({ cheese: false, mushroom: false, pepper: false, onion: false });

    formKeys = Object.keys(this.model()) as (keyof Toppings)[];

    exampleForm = form(this.model, (path) => {
        const atLeastOne = ({ state }: any) => {
            if (!state.touched()) return null;

            const t = this.model();

            return Object.values(t).some(Boolean) ? null : { kind: 'atLeastOneRequired', message: 'At least one ingredient must be selected.' };
        };

        validate(path, atLeastOne);

        for (const key of this.formKeys) {
            validate(path[key], atLeastOne);
        }
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

Checkbox provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-wrap items-center justify-center gap-4">
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="small" [binary]="true" inputId="small-checkbox" size="small" />
                <label pLabel for="small-checkbox" class="text-sm">Small</label>
            </div>
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="normal" [binary]="true" inputId="normal-checkbox" />
                <label pLabel for="normal-checkbox" class="text-sm">Normal</label>
            </div>
            <div class="flex items-center gap-2">
                <p-checkbox [(ngModel)]="large" [binary]="true" inputId="large-checkbox" size="large" />
                <label pLabel for="large-checkbox" class="text-sm">Large</label>
            </div>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, LabelModule, FormsModule]
})
export class CheckboxSizesDemo {
    small: boolean = false;

    normal: boolean = false;

    large: boolean = false;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #form="ngForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4">
                <div class="flex flex-wrap gap-4">
                    @for (item of formKeys; track item) {
                        <div class="flex items-center gap-2">
                            <p-checkbox [inputId]="'td_' + item" [name]="item" [(ngModel)]="formModel[item]" [binary]="true" [invalid]="isInvalid()"></p-checkbox>
                            <label pLabel [for]="'td_' + item" class="text-sm">{{ item | titlecase }}</label>
                        </div>
                    }
                </div>
                @if (isInvalid()) {
                    <p-message severity="error" size="small" variant="simple"> At least one ingredient must be selected. </p-message>
                }
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, MessageModule, ToastModule, ButtonModule, LabelModule, FormsModule]
})
export class CheckboxTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    formSubmitted: boolean = false;

    formModel = {
        cheese: false,
        mushroom: false,
        pepper: false,
        onion: false
    };

    get formKeys(): string[] {
        return Object.keys(this.formModel);
    }

    isInvalid(): boolean {
        return this.formSubmitted && !this.isAtLeastOneSelected();
    }

    isAtLeastOneSelected(): boolean {
        return Object.values(this.formModel).some((value) => value === true);
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.isAtLeastOneSelected()) {
            this.messageService.add({
                severity: 'success',
                summary: 'Success',
                detail: 'Form is submitted',
                life: 3000
            });

            this.formModel = {
                cheese: false,
                mushroom: false,
                pepper: false,
                onion: false
            };
            this.formSubmitted = false;
        }
    }
}
```

## Checkbox

Checkbox is an extension to standard checkbox element with theming.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, CheckboxPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| value | any | - | Value of the checkbox. |
| binary | boolean | - | Allows to select a boolean value instead of multiple values. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| ariaLabel | string \| undefined | - | Used to define a string that labels the input element. |
| tabindex | number \| undefined | - | Index of the element in tabbing order. |
| inputId | string \| undefined | - | Identifier of the focus input to match a label defined for the component. |
| inputStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the input element. |
| inputClass | string \| undefined | - | Style class of the input element. |
| indeterminate | boolean | - | When present, it specifies input state as indeterminate. |
| formControl | FormControl<any> \| undefined | - | Form control value. |
| checkboxIcon | string \| undefined | - | Icon class of the checkbox icon. |
| readonly | boolean | - | When present, it specifies that the component cannot be edited. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| trueValue | any | - | Value in checked state. |
| falseValue | any | - | Value in unchecked state. |
| variant | InputVariant \| undefined | undefined | Specifies the input variant of the component. |
| size | InputSize \| undefined | undefined | Specifies the size of the component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: CheckboxChangeEvent | Callback to invoke on value change. |
| onFocus | event: Event | Callback to invoke when the receives focus. |
| onBlur | event: Event | Callback to invoke when the loses focus. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| icon | TemplateRef<CheckboxIconTemplateContext> \| undefined | Custom checkbox icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |
| box | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the box's DOM element. |
| indicator | PassThroughOption<HTMLElement, I> | Used to pass attributes to the indicator's DOM element. |
| icon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-checkbox | Class name of the root element |
| p-checkbox-box | Class name of the box element |
| p-checkbox-input | Class name of the input element |
| p-checkbox-indicator | Class name of the indicator element |
| p-checkbox-icon | Class name of the icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| checkbox.border.radius | --p-checkbox-border-radius | Border radius of root |
| checkbox.width | --p-checkbox-width | Width of root |
| checkbox.height | --p-checkbox-height | Height of root |
| checkbox.background | --p-checkbox-background | Background of root |
| checkbox.checked.background | --p-checkbox-checked-background | Checked background of root |
| checkbox.checked.hover.background | --p-checkbox-checked-hover-background | Checked hover background of root |
| checkbox.disabled.background | --p-checkbox-disabled-background | Disabled background of root |
| checkbox.filled.background | --p-checkbox-filled-background | Filled background of root |
| checkbox.border.color | --p-checkbox-border-color | Border color of root |
| checkbox.hover.border.color | --p-checkbox-hover-border-color | Hover border color of root |
| checkbox.focus.border.color | --p-checkbox-focus-border-color | Focus border color of root |
| checkbox.checked.border.color | --p-checkbox-checked-border-color | Checked border color of root |
| checkbox.checked.hover.border.color | --p-checkbox-checked-hover-border-color | Checked hover border color of root |
| checkbox.checked.focus.border.color | --p-checkbox-checked-focus-border-color | Checked focus border color of root |
| checkbox.checked.disabled.border.color | --p-checkbox-checked-disabled-border-color | Checked disabled border color of root |
| checkbox.invalid.border.color | --p-checkbox-invalid-border-color | Invalid border color of root |
| checkbox.shadow | --p-checkbox-shadow | Shadow of root |
| checkbox.focus.ring.width | --p-checkbox-focus-ring-width | Focus ring width of root |
| checkbox.focus.ring.style | --p-checkbox-focus-ring-style | Focus ring style of root |
| checkbox.focus.ring.color | --p-checkbox-focus-ring-color | Focus ring color of root |
| checkbox.focus.ring.offset | --p-checkbox-focus-ring-offset | Focus ring offset of root |
| checkbox.focus.ring.shadow | --p-checkbox-focus-ring-shadow | Focus ring shadow of root |
| checkbox.transition.duration | --p-checkbox-transition-duration | Transition duration of root |
| checkbox.sm.width | --p-checkbox-sm-width | Sm width of root |
| checkbox.sm.height | --p-checkbox-sm-height | Sm height of root |
| checkbox.lg.width | --p-checkbox-lg-width | Lg width of root |
| checkbox.lg.height | --p-checkbox-lg-height | Lg height of root |
| checkbox.icon.size | --p-checkbox-icon-size | Size of icon |
| checkbox.icon.color | --p-checkbox-icon-color | Color of icon |
| checkbox.icon.checked.color | --p-checkbox-icon-checked-color | Checked color of icon |
| checkbox.icon.checked.hover.color | --p-checkbox-icon-checked-hover-color | Checked hover color of icon |
| checkbox.icon.disabled.color | --p-checkbox-icon-disabled-color | Disabled color of icon |
| checkbox.icon.sm.size | --p-checkbox-icon-sm-size | Sm size of icon |
| checkbox.icon.lg.size | --p-checkbox-icon-lg-size | Lg size of icon |
