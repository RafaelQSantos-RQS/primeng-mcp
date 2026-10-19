---
title: "Angular InputMask Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inputmask.md"
description: "InputMask component is used to enter input in a certain format such as numeric, date, currency and phone."
---

# Angular InputMask Component

InputMask component is used to enter input in a certain format such as numeric, date, currency and phone.

## Accessibility

Screen Reader InputMask directive is used with a native input element that implicitly includes any passed attribute. Value to describe the component can either be provided via label tag combined with id attribute or using aria-labelledby , aria-label attributes.

## AutoClear

When autoClear is set to false, the incomplete value is preserved on blur instead of being cleared.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputText [(ngModel)]="value" pInputMask="99-999999" placeholder="99-999999" [autoClear]="false" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskAutoclearDemo {
    value: string | undefined;
}
```

## Basic

InputMask is used as a controlled input with ngModel properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputText pInputMask="99-999999" [(ngModel)]="value" placeholder="99-999999" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskBasicDemo {
    value: string | undefined;
}
```

## clearicon-doc

When showClear is enabled, a clear icon is displayed to clear the value.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputmask mask="99-999999" [(ngModel)]="value" placeholder="99-999999" [showClear]="true" />
        </div>
    `,
    standalone: true,
    imports: [InputMaskModule, FormsModule]
})
export class InputMaskClearIconDemo {
    value: string | undefined;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputText pInputMask="999-99-9999" [(ngModel)]="value" disabled />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskDisabledDemo {
    value: string | undefined;
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputText pInputMask="99-999999" [(ngModel)]="value" variant="filled" placeholder="99-999999" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskFilledDemo {
    value: string | undefined;
}
```

## Float Label

FloatLabel visually integrates a label with its form element. Visit FloatLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FloatLabelModule } from 'primeng/floatlabel';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex flex-wrap justify-center items-end gap-4">
            <p-floatlabel>
                <input pInputText id="over_label" [(ngModel)]="value1" pInputMask="999-99-9999" />
                <label for="over_label">Over Label</label>
            </p-floatlabel>
            <p-floatlabel variant="in">
                <input pInputText id="in_label" [(ngModel)]="value2" pInputMask="999-99-9999" />
                <label for="in_label">In Label</label>
            </p-floatlabel>
            <p-floatlabel variant="on">
                <input pInputText id="on_label" [(ngModel)]="value3" pInputMask="999-99-9999" />
                <label for="on_label">On Label</label>
            </p-floatlabel>
        </div>
    `,
    standalone: true,
    imports: [FloatLabelModule, InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskFloatLabelDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## Fluid

The fluid prop makes the component take up the full width of its container when set to true.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <input pInputText pInputMask="99-999999" [(ngModel)]="value" placeholder="99-999999" fluid />
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskFluidDemo {
    value: string | undefined;
}
```

## Format Patterns

Mask format can be a combination of the following definitions; a for alphabetic characters, 9 for numeric characters and * for alphanumberic characters. In addition, formatting characters like ( , ) , - are also accepted.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FluidModule } from 'primeng/fluid';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <p-fluid class="flex flex-wrap gap-4">
            <div class="flex-auto">
                <label pLabel for="ssn" class="font-bold! block mb-2">SSN</label>
                <input pInputText id="ssn" pInputMask="999-99-9999" [(ngModel)]="value1" placeholder="999-99-9999" />
            </div>
            <div class="flex-auto">
                <label pLabel for="phone" class="font-bold! block mb-2">Phone</label>
                <input pInputText id="phone" pInputMask="(999) 999-9999" [(ngModel)]="value2" placeholder="(999) 999-9999" />
            </div>
            <div class="flex-auto">
                <label pLabel for="serial" class="font-bold! block mb-2">Serial</label>
                <input pInputText id="serial" pInputMask="a*-999-a999" [(ngModel)]="value3" placeholder="a*-999-a999" />
            </div>
        </p-fluid>
    `,
    standalone: true,
    imports: [FluidModule, InputTextModule, InputMaskModule, LabelModule, FormsModule]
})
export class InputMaskFormatPatternsDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## Ifta Label

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iftalabel>
                <input pInputText id="ssn" [(ngModel)]="value" pInputMask="999-99-9999" autocomplete="off" />
                <label for="ssn">SSN</label>
            </p-iftalabel>
        </div>
    `,
    standalone: true,
    imports: [IftaLabelModule, InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskIftaLabelDemo {
    value: string | undefined;
}
```

## Invalid

The invalid state is applied using the invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <input pInputText [(ngModel)]="value1" pInputMask="99-999999" placeholder="Serial Key" [invalid]="!value1" />
            <input pInputText [(ngModel)]="value2" pInputMask="99-999999" placeholder="Serial Key" [invalid]="!value2" variant="filled" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskInvalidDemo {
    value1: string | undefined;

    value2: string | undefined;
}
```

## Optional

When the input does not complete the mask definition, it is cleared by default. Use autoClear property to control this behavior. In addition, ? is used to mark anything after the question mark optional.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <input pInputText pInputMask="(999) 999-9999? x99999" [(ngModel)]="value" placeholder="(999) 999-9999? x99999" />
            <span class="text-sm text-muted-color">Bound value: {{ value || '—' }}</span>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskOptionalDemo {
    value: string | undefined;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="max-w-md mx-auto flex flex-col gap-4">
            <div class="flex flex-col gap-2">
                <label pLabel for="prev-phone" class="font-bold!">Phone</label>
                <input pInputText id="prev-phone" [(ngModel)]="phone" pInputMask="(999) 999-9999" placeholder="(999) 999-9999" class="w-full" />
            </div>
            <div class="flex flex-col gap-2">
                <label pLabel for="prev-date" class="font-bold!">Date</label>
                <input pInputText id="prev-date" [(ngModel)]="date" pInputMask="99/99/9999" slotChar="dd/mm/yyyy" placeholder="dd/mm/yyyy" class="w-full" />
            </div>
            <div class="flex flex-col gap-2">
                <label pLabel for="prev-card" class="font-bold!">Card Number</label>
                <input pInputText id="prev-card" [(ngModel)]="card" pInputMask="9999 9999 9999 9999" placeholder="9999 9999 9999 9999" class="w-full" />
            </div>
            <div class="flex flex-col gap-2">
                <label pLabel for="prev-ssn" class="font-bold!">SSN</label>
                <input pInputText id="prev-ssn" [(ngModel)]="ssn" pInputMask="999-99-9999" placeholder="999-99-9999" class="w-full" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, LabelModule, FormsModule]
})
export class InputMaskPreviewDemo {
    phone: string | undefined;

    date: string | undefined;

    card: string | undefined;

    ssn: string | undefined;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 sm:w-56">
                <div class="flex flex-col gap-1">
                    <input pInputText pInputMask="99-999999" formControlName="value" placeholder="99-999999" [invalid]="isInvalid('value')" fluid />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">Serial number is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ButtonModule, InputTextModule, InputMaskModule, ReactiveFormsModule]
})
export class InputMaskReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    items: any[] | undefined;

    exampleForm: FormGroup | undefined;

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

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 sm:w-56">
                <div class="flex flex-col gap-1">
                    <input pInputText pInputMask="99-999999" [formField]="exampleForm.value" placeholder="99-999999" fluid />
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
    imports: [MessageModule, ToastModule, ButtonModule, InputTextModule, InputMaskModule, FormField]
})
export class InputMaskSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ value: '' });

    exampleForm = form(this.model, (path) => {
        required(path.value, { when: ({ state }) => state.touched(), message: 'Serial number is required.' });
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

InputMask provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <input pInputText [(ngModel)]="value1" placeholder="Small" pSize="small" pInputMask="99-999999" />
            <input pInputText [(ngModel)]="value2" placeholder="Normal" pInputMask="99-999999" />
            <input pInputText [(ngModel)]="value3" placeholder="Large" pSize="large" pInputMask="99-999999" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskSizesDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## Slot Character

Default placeholder for a mask is underscore that can be customized using slotChar property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex justify-center">
            <input pInputText [(ngModel)]="value" pInputMask="99/99/9999" placeholder="99/99/9999" slotChar="mm/dd/yyyy" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskSlotCharDemo {
    value: string | undefined;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex justify-center flex-col gap-4 md:w-56">
                <div class="flex flex-col gap-1">
                    <input pInputText name="serial" pInputMask="99-999999" #serialNumber="ngModel" [(ngModel)]="value" placeholder="99-999999" [invalid]="serialNumber.invalid && (serialNumber.touched || exampleForm.submitted)" required fluid />
                    @if (serialNumber.invalid && (serialNumber.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Serial number is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ButtonModule, InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    items: any[] = [];

    value: any;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Unmask

The onUnmaskedChange event is invoked on every value change and emits the rawValue without the mask formatting characters, so both the masked and raw values can be accessed at the same time.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { InputMaskModule } from 'primeng/inputmask';

@Component({
    template: `
        <div class="flex flex-col items-center gap-2">
            <input pInputText [(ngModel)]="value" pInputMask="(999) 999-9999" placeholder="(999) 999-9999" (onUnmaskedChange)="rawValue = $event" />
            <span class="text-sm text-muted-color">Masked: {{ value || '—' }}</span>
            <span class="text-sm text-muted-color">Raw: {{ rawValue || '—' }}</span>
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, InputMaskModule, FormsModule]
})
export class InputMaskUnmaskDemo {
    value: string | undefined;

    rawValue: string | undefined;
}
```

## Input Mask

InputMask component is used to enter input in a certain format such as numeric, date, currency, email and phone.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InputMaskPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| fluid | boolean \| undefined | false | Spans 100% width of the container when enabled. |
| variant | "filled" \| "outlined" \| undefined | 'outlined' | Specifies the input variant of the component. |
| size | "small" \| "large" \| undefined | undefined | Specifies the size of the component. |
| inputSize | number \| null \| undefined | undefined | Specifies the visible width of the input element in characters. |
| pattern | string \| readonly RegExp[] \| null \| undefined | undefined | Specifies the value must match the pattern. |
| min | string \| null \| undefined | undefined | The value must be greater than or equal to the value. |
| max | string \| null \| undefined | undefined | The value must be less than or equal to the value. |
| step | number \| null \| undefined | undefined | Unless the step is set to the any literal, the value must be min + an integral multiple of the step. |
| minlength | number \| null \| undefined | undefined | The number of characters (code points) must not be less than the value of the attribute, if non-empty. |
| maxlength | number \| null \| undefined | undefined | The number of characters (code points) must not exceed the value of the attribute. |
| type | string | - | HTML5 input type. |
| slotChar | string | - | Placeholder character in mask, default is underscore. |
| autoClear | boolean | - | Clears the incomplete value on blur. |
| showClear | boolean | - | When enabled, a clear icon is displayed to clear the value. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the input field. |
| inputId | string \| undefined | - | Identifier of the focus input to match a label defined for the component. |
| styleClass | string \| undefined | - | Style class of the input field. |
| placeholder | string \| undefined | - | Advisory information to display on input. |
| tabindex | string \| undefined | - | Specifies tab order of the element. |
| title | string \| undefined | - | Title text of the input text. |
| ariaLabel | string \| undefined | - | Used to define a string that labels the input element. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| ariaRequired | boolean \| undefined | - | Used to indicate that user input is required on an element before a form can be submitted. |
| readonly | boolean | - | When present, it specifies that an input field is read-only. |
| unmask | boolean | - | Defines if ngModel sets the raw unmasked value to bound value or the formatted mask value. |
| characterPattern | string | - | Regex pattern for alpha characters |
| autofocus | boolean | - | When present, the input gets a focus automatically on load. |
| autocomplete | string \| undefined | - | Used to define a string that autocomplete attribute the current element. |
| keepBuffer | boolean | - | When present, it specifies that whether to clean buffer value from model. |
| keepCharPositions | boolean | - | When enabled, deleting a character does not shift the following characters across literal separators (e.g. the "." in 99.99.9999), keeping each mask section independent. |
| mask | string \| null \| undefined | - | Mask pattern. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onComplete | value: void | Callback to invoke when the mask is completed. |
| onFocus | event: Event | Callback to invoke when the component receives focus. |
| onBlur | event: Event | Callback to invoke when the component loses focus. |
| onInput | event: Event | Callback to invoke on input. |
| onKeydown | event: Event | Callback to invoke on input key press. |
| onClear | value: void | Callback to invoke when input field is cleared. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| clearicon | TemplateRef<void> \| undefined | Custom clear icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the root's DOM element. |
| pcInputText | InputTextPassThrough | Used to pass attributes to the InputText component. |
| clearIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the clear icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inputmask | Class name of the root element |
| p-inputmask-clear-icon | Class name of the clear icon element |
