---
title: "Angular Slider Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/slider.md"
description: "Slider is a component to provide input by dragging a handle along a track."
---

# Angular Slider Component

Slider is a component to provide input by dragging a handle along a track.

## Accessibility

Screen Reader Slider element component uses slider role on the handle in addition to the aria-orientation , aria-valuemin , aria-valuemax and aria-valuenow attributes. Value to describe the component can be defined using ariaLabelledBy and ariaLabel props.

## Basic

Select a numeric value by dragging a handle along a track. Two-way binding is defined using the standard ngModel directive.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <p-slider [(ngModel)]="value" class="max-w-3xs mx-auto w-full" />
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderBasicDemo {
    value: number = 50;
}
```

## Controlled

Slider is connected to an InputNumber using two-way binding.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputNumberModule } from 'primeng/inputnumber';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="max-w-3xs w-full mx-auto">
            <p-inputnumber [(ngModel)]="value" [showButtons]="false" fluid class="mb-4" />
            <p-slider [(ngModel)]="value" class="w-full" />
        </div>
    `,
    standalone: true,
    imports: [InputNumberModule, SliderModule, FormsModule]
})
export class SliderControlledDemo {
    value: number = 50;
}
```

## Custom

Slider is customized with the pt property to override the style of each section.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="max-w-xs mx-auto w-full space-y-16">
            <p-slider
                [(ngModel)]="value1"
                [pt]="{
                    track: { class: 'h-2! bg-blue-100! dark:bg-blue-950!' },
                    range: { class: 'bg-blue-500!' },
                    handle: { class: 'before:hidden! border-2! border-blue-500! bg-surface-0! dark:bg-surface-900! rotate-45! rounded-md!' }
                }"
            />
            <p-slider
                [(ngModel)]="value2"
                [pt]="{
                    track: { class: 'h-2! bg-amber-100! dark:bg-amber-950!' },
                    range: { class: 'bg-amber-500!' },
                    handle: { class: 'before:hidden! w-4! h-5! border-2! border-amber-500! bg-surface-0! dark:bg-surface-900! rounded-sm!' }
                }"
            />
        </div>
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderCustomDemo {
    value1: number = 50;

    value2: number = 50;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="max-w-3xs w-full mx-auto space-y-12">
            <div>
                <h5 class="mb-2 text-sm font-medium">Disabled Slider</h5>
                <p-slider [(ngModel)]="value" [disabled]="true" />
            </div>
            <div>
                <h5 class="mb-2 text-sm font-medium">Disabled All Handles</h5>
                <p-slider [(ngModel)]="rangeValues" [range]="true" [disabled]="true" />
            </div>
            <div>
                <h5 class="mb-2 text-sm font-medium">Disabled Single Handle</h5>
                <p-slider [(ngModel)]="rangeValues2" [range]="true" [disabledMinHandle]="true" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderDisabledDemo {
    value: number = 50;

    rangeValues: number[] = [20, 80];

    rangeValues2: number[] = [20, 80];
}
```

## Filter

Image filter implementation using multiple sliders.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectButtonModule } from 'primeng/selectbutton';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="flex flex-col items-center justify-center">
            <img alt="user header" class="w-80 rounded mb-6" src="https://primefaces.org/cdn/primevue/images/galleria/galleria1.jpg" [style]="filterStyle" />
            <p-selectbutton [(ngModel)]="filter" [options]="filterOptions" optionLabel="label" optionValue="value" />
            <p-slider [(ngModel)]="filterValues[filter]" class="max-w-56 mt-4" [min]="0" [max]="200" />
        </div>
    `,
    standalone: true,
    imports: [SelectButtonModule, SliderModule, FormsModule]
})
export class SliderFilterDemo {
    filter: number = 0;

    filterValues: number[] = [100, 100, 0];

    filterOptions: any = [
        { label: 'Contrast', value: 0 },
        { label: 'Brightness', value: 1 },
        { label: 'Sepia', value: 2 }
    ];

    get filterStyle() {
        return {
            filter: `contrast(${this.filterValues[0]}%) brightness(${this.filterValues[1]}%) sepia(${this.filterValues[2]}%)`
        };
    }
}
```

## Handles Distance

The minStepsBetweenHandles property defines the minimum number of steps between handles in range mode.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <p-slider [(ngModel)]="rangeValues" [range]="true" [minStepsBetweenHandles]="20" class="max-w-3xs mx-auto w-full" />
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderHandlesdistanceDemo {
    rangeValues: number[] = [20, 80];
}
```

## input-doc

Slider is connected to an input field using two-way binding.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="max-w-3xs w-full mx-auto">
            <input type="text" pInputText [(ngModel)]="value" class="w-full mb-4" />
            <p-slider [(ngModel)]="value" class="w-full" />
        </div>
    `,
    standalone: true,
    imports: [SliderModule, InputTextModule, FormsModule]
})
export class SliderInputDemo {
    value: number = 50;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="flex items-center gap-4 justify-center mx-auto w-full">
            <img class="h-20 w-auto rounded-lg" src="https://images.unsplash.com/photo-1531777992189-ad52457fbe93?w=200&h=200&fit=crop&auto=format" alt="Ivory by Soft Landing" />
            <div class="w-full max-w-3xs">
                <div>
                    <span class="text-lg font-bold text-color">Ivory</span>
                    <div class="text-surface-500">Soft Landing</div>
                </div>
                <div class="flex items-center gap-4 mt-2">
                    <span class="text-sm tabular-nums text-surface-500">{{ currentTime }}</span>
                    <p-slider [(ngModel)]="progress" [min]="0" [max]="225" class="flex-1" />
                    <span class="text-sm tabular-nums text-surface-500">3:45</span>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderPreviewDemo {
    progress: number = 128;

    get currentTime(): string {
        const totalSeconds = Math.floor(this.progress);
        const minutes = Math.floor(totalSeconds / 60);
        const seconds = totalSeconds % 60;

        return `${minutes}:${String(seconds).padStart(2, '0')}`;
    }
}
```

## Range

When the range property is present, slider provides two handles to define two values. In range mode, the value should be an array instead of a single value.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <p-slider [(ngModel)]="rangeValues" [range]="true" class="max-w-3xs mx-auto w-full" />
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderRangeDemo {
    rangeValues: number[] = [20, 80];
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { SliderModule } from 'primeng/slider';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 max-w-3xs w-full">
                <div class="flex flex-col gap-4">
                    <p-slider formControlName="value" class="w-full" />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">Must be greater than 25.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, SliderModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class SliderReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

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
import { form, FormField, submit, validate } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { SliderModule } from 'primeng/slider';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 max-w-3xs w-full">
                <div class="flex flex-col gap-4">
                    <p-slider [formField]="exampleForm.value" class="w-full" />
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
    imports: [MessageModule, SliderModule, ToastModule, ButtonModule, FormField]
})
export class SliderSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal<{ value: number | null }>({ value: null });

    exampleForm = form(this.model, (path) => {
        validate(path.value, ({ value, state }) => {
            if (!state.touched()) return null;

            const v = value();

            if (v == null || v <= 25) return { kind: 'min', message: 'Must be greater than 25.' };

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

## Step

Size of each movement is defined with the step property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <p-slider [(ngModel)]="value" [step]="20" class="max-w-3xs mx-auto w-full" />
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderStepDemo {
    value: number = 20;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { SliderModule } from 'primeng/slider';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 max-w-3xs w-full">
                <div class="flex flex-col gap-4">
                    <p-slider #model="ngModel" [(ngModel)]="value" class="w-full" required [invalid]="model.invalid && (model.touched || exampleForm.submitted)" name="slider" />
                    @if (model.invalid && (model.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Must be greater than 25.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, SliderModule, ToastModule, ButtonModule, FormsModule]
})
export class SliderTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: any;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Value Change

The onChange event is triggered during drag, while onSlideEnd is fired when the drag operation completes.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="max-w-3xs mx-auto w-full flex flex-col items-center space-y-4">
            <div class="text-sm text-surface-500 font-mono">onChange: {{ value }}</div>
            <div class="text-sm text-surface-500 font-mono">onSlideEnd: {{ endValue }}</div>
            <p-slider [(ngModel)]="value" (onChange)="onChangeHandler($event)" (onSlideEnd)="onSlideEndHandler($event)" class="w-full" />
        </div>
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderValuechangeDemo {
    value: number = 50;

    endValue: number = 50;

    onChangeHandler(event: SliderChangeEvent) {
        this.value = event.value as number;
    }

    onSlideEndHandler(event: SliderSlideEndEvent) {
        this.endValue = event.value as number;
    }
}
```

## Vertical

Default layout of slider is horizontal , use orientation property for the alternative vertical mode.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SliderModule } from 'primeng/slider';

@Component({
    template: `
        <div class="grid grid-cols-3 h-56 gap-4 w-fit mx-auto">
            @for (slider of sliders; track slider.label; let i = $index) {
                <div class="flex flex-col items-center justify-center gap-3">
                    <p-slider [(ngModel)]="slider.value" orientation="vertical" />
                    <span class="font-mono text-xs uppercase text-surface-500">{{ slider.label }}</span>
                </div>
            }
        </div>
    `,
    standalone: true,
    imports: [SliderModule, FormsModule]
})
export class SliderVerticalDemo {
    sliders = [
        { label: 'Bass', value: 40 },
        { label: 'Mid', value: 70 },
        { label: 'Treble', value: 55 }
    ];
}
```

## Slider

Slider is a component to provide input by dragging a handle along a track.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SliderPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| animate | boolean \| undefined | - | When enabled, displays an animation on click of the slider bar. **(Deprecated)** |
| min | number | - | Mininum boundary value. |
| max | number | - | Maximum boundary value. |
| orientation | "horizontal" \| "vertical" | - | Orientation of the slider. |
| step | number \| undefined | - | Step factor to increment/decrement the value. |
| range | boolean \| undefined | - | When specified, allows two boundary values to be picked. |
| minStepsBetweenHandles | number | - | Minimum number of steps between handles in range mode. |
| disabledMinHandle | boolean | false | Disables the minimum (start) handle in range mode. |
| disabledMaxHandle | boolean | false | Disables the maximum (end) handle in range mode. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| tabindex | number | - | Index of the element in tabbing order. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: SliderChangeEvent | Callback to invoke on value change. |
| onSlideEnd | event: SliderSlideEndEvent | Callback to invoke when slide ended. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| track | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the track's DOM element. |
| range | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the range's DOM element. |
| handle | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the handle's DOM element. |
| startHandler | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the start handler's DOM element. |
| endHandler | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the end handler's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the hidden input's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-slider | Class name of the root element |
| p-slider-track | Class name of the track element |
| p-slider-range | Class name of the range element |
| p-slider-handle | Class name of the handle element |
| p-slider-input | Class name of the hidden input element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| slider.transition.duration | --p-slider-transition-duration | Transition duration of root |
| slider.track.background | --p-slider-track-background | Background of track |
| slider.track.border.radius | --p-slider-track-border-radius | Border radius of track |
| slider.track.size | --p-slider-track-size | Size of track |
| slider.range.background | --p-slider-range-background | Background of range |
| slider.handle.width | --p-slider-handle-width | Width of handle |
| slider.handle.height | --p-slider-handle-height | Height of handle |
| slider.handle.border.radius | --p-slider-handle-border-radius | Border radius of handle |
| slider.handle.background | --p-slider-handle-background | Background of handle |
| slider.handle.hover.background | --p-slider-handle-hover-background | Hover background of handle |
| slider.handle.content.border.radius | --p-slider-handle-content-border-radius | Content border radius of handle |
| slider.handle.content.background | --p-slider-handle-content-background | Background of handle |
| slider.handle.content.hover.background | --p-slider-handle-content-hover-background | Content hover background of handle |
| slider.handle.content.width | --p-slider-handle-content-width | Content width of handle |
| slider.handle.content.height | --p-slider-handle-content-height | Content height of handle |
| slider.handle.content.shadow | --p-slider-handle-content-shadow | Content shadow of handle |
| slider.handle.focus.ring.width | --p-slider-handle-focus-ring-width | Focus ring width of handle |
| slider.handle.focus.ring.style | --p-slider-handle-focus-ring-style | Focus ring style of handle |
| slider.handle.focus.ring.color | --p-slider-handle-focus-ring-color | Focus ring color of handle |
| slider.handle.focus.ring.offset | --p-slider-handle-focus-ring-offset | Focus ring offset of handle |
| slider.handle.focus.ring.shadow | --p-slider-handle-focus-ring-shadow | Focus ring shadow of handle |
