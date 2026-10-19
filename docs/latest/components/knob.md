---
title: "Angular Knob Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/knob.md"
description: "Knob is a form component to define number inputs with a dial."
---

# Angular Knob Component

Knob is a form component to define number inputs with a dial.

## Accessibility

Screen Reader Knob element component uses slider role in addition to the aria-valuemin , aria-valuemax and aria-valuenow attributes. Value to describe the component can be defined using ariaLabelledBy and ariaLabel props.

## Basic

Knob is an input component and used with the standard ngModel directive.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobBasicDemo {
    value: number = 50;
}
```

## Color

Colors are customized with the textColor , rangeColor and valueColor properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" valueColor="PeachPuff" rangeColor="MediumTurquoise" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobColorDemo {
    value: number = 50;
}
```

## Disabled

When disabled is present, a visual hint is applied to indicate that the Knob cannot be interacted with.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobDisabledDemo {
    value: number = 75;
}
```

## Min/Max

Boundaries are configured with the min and max properties whose defaults are 0 and 100 respectively.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [min]="-50" [max]="50" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobMinMaxDemo {
    value: number = 10;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex flex-wrap gap-8 items-center justify-center">
            <div class="flex flex-col items-center gap-2">
                <p-knob [(ngModel)]="value1" />
                <span class="text-sm text-muted-color font-medium">Volume</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <p-knob [(ngModel)]="value2" />
                <span class="text-sm text-muted-color font-medium">Treble</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <p-knob [(ngModel)]="value3" />
                <span class="text-sm text-muted-color font-medium">Bass</span>
            </div>
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobPreviewDemo {
    value1: number = 48;

    value2: number = 75;

    value3: number = 24;
}
```

## Reactive

Knob can be controlled with custom controls as well.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';
import { ButtonModule } from 'primeng/button';
import { Plus } from '@primeicons/angular/plus';
import { Minus } from '@primeicons/angular/minus';

@Component({
    template: `
        <div class="flex flex-col items-center gap-2">
            <p-knob [(ngModel)]="value" [size]="150" [readonly]="true" />
            <div class="flex gap-2">
                <button pButton [disabled]="value >= 100" (click)="value = value + 1" iconOnly><svg data-p-icon="plus"></svg></button>
                <button pButton [disabled]="value <= 0" (click)="value = value - 1" iconOnly><svg data-p-icon="minus"></svg></button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [KnobModule, ButtonModule, FormsModule, Plus, Minus]
})
export class KnobReactiveDemo {
    value: number = 0;
}
```

## Reactive Forms

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col items-center gap-4">
                <div class="flex flex-col items-center gap-1">
                    <p-knob formControlName="value" [invalid]="isInvalid('value')" />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">{{ getErrorMessage('value') }}</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [KnobModule, MessageModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class KnobReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    items: any[] | undefined;

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            value: [15, [Validators.min(25), Validators.max(75)]]
        });
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        }
    }

    getControl(controlName: string): AbstractControl | null {
        return this.exampleForm?.get(controlName) ?? null;
    }

    getErrorMessage(controlName: string): string | null {
        const control = this.getControl(controlName);

        if (!control || !control.errors) return null;

        if (control.errors['min']) {
            return 'Value must be greater than 15.';
        }

        if (control.errors['max']) {
            return 'Must be less than 75.';
        }
    }

    isInvalid(controlName: string) {
        const control = this.getControl(controlName);

        return control?.invalid && (control.dirty || this.formSubmitted);
    }
}
```

## ReadOnly

When readonly present, value cannot be edited.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [readonly]="true" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobReadonlyDemo {
    value: number = 50;
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, submit, validate } from '@angular/forms/signals';
import { KnobModule } from 'primeng/knob';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 items-center">
                <div class="flex flex-col gap-1 items-center">
                    <p-knob [formField]="exampleForm.value" />
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
    imports: [KnobModule, MessageModule, ToastModule, ButtonModule, FormField]
})
export class KnobSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ value: 15 });

    exampleForm = form(this.model, (path) => {
        validate(path.value, ({ value, state }) => {
            if (!state.touched()) return null;

            const v = value();

            if (v == null) return null;

            if (v < 25) return { kind: 'min', message: 'Value must be at least 25.' };

            if (v > 75) return { kind: 'max', message: 'Value must be at most 75.' };

            return null;
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

## Size

Diameter of the knob is defined in pixels using the size property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [size]="200" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobSizeDemo {
    value: number = 60;
}
```

## Step

Size of each movement is defined with the step property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [step]="10" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobStepDemo {
    value: number = 50;
}
```

## Stroke

The border size is specified with the strokeWidth property as a number in pixels.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" [strokeWidth]="5" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobStrokeDemo {
    value: number = 40;
}
```

## Template

Label is a string template that can be customized with the valueTemplate property having 60 as the placeholder .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';

@Component({
    template: `
        <div class="flex justify-center">
            <p-knob [(ngModel)]="value" valueTemplate="{value}%" />
        </div>
    `,
    standalone: true,
    imports: [KnobModule, FormsModule]
})
export class KnobTemplateDemo {
    value: number = 60;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { KnobModule } from 'primeng/knob';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col items-center gap-4">
                <div class="flex flex-col items-center gap-1">
                    <p-knob #model="ngModel" [(ngModel)]="value" [invalid]="isInvalid(model)" name="knob" />
                    @if (isInvalid(model)) {
                        <p-message severity="error" size="small" variant="simple">{{ getErrorMessage(model) }}</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [KnobModule, MessageModule, ToastModule, ButtonModule, FormsModule]
})
export class KnobTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: number = 15;

    formSubmitted: boolean = false;

    onSubmit(form: NgForm) {
        this.formSubmitted = true;

        if (!this.isInvalid(form.controls['knob'])) {
            this.messageService.add({
                severity: 'success',
                summary: 'Success',
                detail: 'Form is submitted',
                life: 3000
            });

            this.formSubmitted = false;
        }
    }

    getErrorMessage(control: any): string | null {
        const value = control?.value;

        return value < 25 ? 'Value must be greater than 25.' : value > 75 ? 'Must be less than 75.' : null;
    }

    isInvalid(control: any): boolean {
        if (!control) return false;

        const value = control.value;

        const hasError = value < 25 || value > 75;

        return hasError && (this.formSubmitted || control.dirty);
    }
}
```

## Knob

Knob is a form component to define number inputs with a dial.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, KnobPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| ariaLabelledBy | string \| undefined | - | Specifies one or more IDs in the DOM that labels the input field. |
| tabindex | number | - | Index of the element in tabbing order. |
| valueColor | string | - | Background of the value. |
| rangeColor | string | - | Background color of the range. |
| textColor | string | - | Color of the value text. |
| valueTemplate | string | - | Template string of the value. |
| size | number | - | Size of the component in pixels. |
| min | number | - | Mininum boundary value. |
| max | number | - | Maximum boundary value. |
| step | number | - | Step factor to increment/decrement the value. |
| strokeWidth | number | - | Width of the knob stroke. |
| showValue | boolean | - | Whether the show the value inside the knob. |
| readonly | boolean | - | When present, it specifies that the component value cannot be edited. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | value: number | Callback to invoke on value change. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| svg | PassThroughOption<SVGElement, I> | Used to pass attributes to the SVG's DOM element. |
| range | PassThroughOption<SVGPathElement, I> | Used to pass attributes to the range's DOM element. |
| value | PassThroughOption<SVGPathElement, I> | Used to pass attributes to the value's DOM element. |
| text | PassThroughOption<SVGTextElement, I> | Used to pass attributes to the text's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-knob | Class name of the root element |
| p-knob-range | Class name of the range element |
| p-knob-value | Class name of the value element |
| p-knob-text | Class name of the text element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| knob.transition.duration | --p-knob-transition-duration | Transition duration of root |
| knob.focus.ring.width | --p-knob-focus-ring-width | Focus ring width of root |
| knob.focus.ring.style | --p-knob-focus-ring-style | Focus ring style of root |
| knob.focus.ring.color | --p-knob-focus-ring-color | Focus ring color of root |
| knob.focus.ring.offset | --p-knob-focus-ring-offset | Focus ring offset of root |
| knob.focus.ring.shadow | --p-knob-focus-ring-shadow | Focus ring shadow of root |
| knob.value.background | --p-knob-value-background | Background of value |
| knob.range.background | --p-knob-range-background | Background of range |
| knob.text.color | --p-knob-text-color | Color of text |
| knob.text.font.size | --p-knob-text-font-size | Font size of text |
| knob.text.font.weight | --p-knob-text-font-weight | Font weight of text |
