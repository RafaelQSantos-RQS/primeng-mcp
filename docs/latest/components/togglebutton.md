---
title: "Angular ToggleButton Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/togglebutton.md"
description: "ToggleButton is used to select a boolean value using a button."
---

# Angular ToggleButton Component

ToggleButton is used to select a boolean value using a button.

## Accessibility

Screen Reader ToggleButton component uses an element with button role and updates aria-pressed state for screen readers. Value to describe the component can be defined with ariaLabelledBy or ariaLabel props, it is highly suggested to use either of these props as the component changes the label displayed which will result in screen readers to read different labels when the component receives focus. To prevent this, always provide an aria label that does not change related to state.

## Basic

Two-way binding to a boolean property is defined using the standard ngModel directive.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';

@Component({
    template: `
        <div class="flex justify-center">
            <p-togglebutton [(ngModel)]="checked" onLabel="On" offLabel="Off" class="w-24" />
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule]
})
export class ToggleButtonBasicDemo {
    checked: boolean = false;
}
```

## Customized

Use the icon template to customize what the component displays based on its internal state.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';
import { Lock } from '@primeicons/angular/lock';
import { LockOpen } from '@primeicons/angular/lock-open';
import { VolumeUp } from '@primeicons/angular/volume-up';
import { VolumeOff } from '@primeicons/angular/volume-off';

@Component({
    template: `
        <div class="flex flex-wrap items-center justify-center gap-4">
            <p-togglebutton [(ngModel)]="value1" onLabel="On" offLabel="Off" class="min-w-16" />
            <p-togglebutton [(ngModel)]="value2" onLabel="Locked" offLabel="Unlocked" class="min-w-29">
                <ng-template #icon let-checked>
                    @if (checked) {
                        <svg data-p-icon="lock" />
                    } @else {
                        <svg data-p-icon="lock-open" />
                    }
                </ng-template>
            </p-togglebutton>
            <p-togglebutton [(ngModel)]="value3" onLabel="Mute" offLabel="Unmute" class="min-w-26">
                <ng-template #icon let-checked>
                    @if (checked) {
                        <svg data-p-icon="volume-up" />
                    } @else {
                        <svg data-p-icon="volume-off" />
                    }
                </ng-template>
            </p-togglebutton>
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule, Lock, LockOpen, VolumeUp, VolumeOff],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToggleButtonCustomizedDemo {
    value1: boolean = false;

    value2: boolean = false;

    value3: boolean = false;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <p-togglebutton [(ngModel)]="checked" [disabled]="true" onLabel="Disabled" offLabel="Disabled" />
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToggleButtonDisabledDemo {
    checked: boolean = false;
}
```

## Fluid

The fluid prop makes the component take up the full width of its container when set to true.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';

@Component({
    template: `
        <div class="flex justify-center">
            <p-togglebutton [(ngModel)]="checked" onLabel="On" offLabel="Off" fluid />
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule]
})
export class ToggleButtonFluidDemo {
    checked: boolean = false;
}
```

## Invalid

The invalid state is applied using the invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <p-togglebutton [(ngModel)]="checked" [invalid]="!checked" onLabel="Invalid" offLabel="Invalid" />
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToggleButtonInvalidDemo {
    checked: boolean = false;
}
```

## preview-doc

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';
import { HeartFill } from '@primeicons/angular/heart-fill';
import { Heart } from '@primeicons/angular/heart';
import { BookmarkFill } from '@primeicons/angular/bookmark-fill';
import { Bookmark } from '@primeicons/angular/bookmark';

@Component({
    template: `
        <div class="max-w-xs mx-auto">
            <div class="w-full text-balance">
                <div class="text-color text-lg font-bold">Migrating to PrimeNG: What Changed and Why</div>
                <div class="mt-1 line-clamp-2 text-muted-color">A hands-on guide to the new compound component API, headless hooks, and Tailwind-first theming in PrimeNG v11.</div>
            </div>
            <div class="mt-4 flex items-start gap-2">
                <div class="flex-1 flex items-center gap-2">
                    <span class="text-muted-color opacity-75">May 12</span>
                </div>
                <p-togglebutton [(ngModel)]="liked" onLabel="Liked" offLabel="Like" class="min-w-24">
                    <ng-template #icon let-checked>
                        @if (checked) {
                            <svg data-p-icon="heart-fill" />
                        } @else {
                            <svg data-p-icon="heart" />
                        }
                    </ng-template>
                </p-togglebutton>
                <p-togglebutton [(ngModel)]="saved" onLabel="Saved" offLabel="Save" class="min-w-24">
                    <ng-template #icon let-checked>
                        @if (checked) {
                            <svg data-p-icon="bookmark-fill" />
                        } @else {
                            <svg data-p-icon="bookmark" />
                        }
                    </ng-template>
                </p-togglebutton>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule, HeartFill, Heart, BookmarkFill, Bookmark],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToggleButtonPreviewDemo {
    liked: boolean = true;

    saved: boolean = false;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ToggleButtonModule } from 'primeng/togglebutton';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col items-center gap-4">
                <div class="flex flex-col items-center gap-1">
                    <p-togglebutton name="consent" formControlName="checked" [invalid]="isInvalid('checked')" onLabel="Accept All" offLabel="Reject All" class="min-w-40" />
                    @if (isInvalid('checked')) {
                        <p-message severity="error" size="small" variant="simple">Consent is mandatory.</p-message>
                    }
                </div>
                <button pButton type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleButtonModule, ButtonModule, ReactiveFormsModule]
})
export class ToggleButtonReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            checked: [false, Validators.requiredTrue]
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
import { ToggleButtonModule } from 'primeng/togglebutton';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col items-center gap-4">
                <div class="flex flex-col items-center gap-1">
                    <p-togglebutton name="consent" [formField]="exampleForm.checked" onLabel="Accept All" offLabel="Reject All" class="min-w-40" />
                    @if (exampleForm.checked().touched() && exampleForm.checked().invalid()) {
                        @for (error of exampleForm.checked().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleButtonModule, ButtonModule, FormField]
})
export class ToggleButtonSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal({ checked: false });

    exampleForm = form(this.model, (path) => {
        validate(path.checked, ({ value, state }) => {
            if (!state.touched()) return null;

            return value() === true ? null : { kind: 'requiredTrue', message: 'Consent is mandatory.' };
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

ToggleButton provides small and large sizes as alternatives to the base.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleButtonModule } from 'primeng/togglebutton';

@Component({
    template: `
        <div class="flex flex-col items-center gap-2">
            <p-togglebutton [(ngModel)]="value1" onLabel="Small" offLabel="Small" size="small" class="min-w-16" />
            <p-togglebutton [(ngModel)]="value2" onLabel="Normal" offLabel="Normal" class="min-w-20" />
            <p-togglebutton [(ngModel)]="value3" onLabel="Large" offLabel="Large" size="large" class="min-w-28" />
        </div>
    `,
    standalone: true,
    imports: [ToggleButtonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToggleButtonSizesDemo {
    value1: boolean = false;

    value2: boolean = false;

    value3: boolean = false;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ToggleButtonModule } from 'primeng/togglebutton';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col items-center gap-4">
                <div class="flex flex-col items-center gap-1">
                    <p-togglebutton #model="ngModel" [(ngModel)]="checked" [invalid]="model.invalid && (model.touched || exampleForm.submitted)" name="country" onLabel="Accept All" offLabel="Reject All" required class="min-w-40" />
                    @if (model.invalid && (model.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Consent is mandatory.</p-message>
                    }
                </div>
                <button pButton type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, ToggleButtonModule, ButtonModule, FormsModule]
})
export class ToggleButtonTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    checked: boolean;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Toggle Button

ToggleButton is used to select a boolean value using a button.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ToggleButtonPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| onLabel | string | - | Label for the on state. |
| offLabel | string | - | Label for the off state. |
| onIcon | string \| undefined | - | Icon for the on state. |
| offIcon | string \| undefined | - | Icon for the off state. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| inputId | string \| undefined | - | Identifier of the focus input to match a label defined for the component. |
| tabindex | number | - | Index of the element in tabbing order. |
| iconPos | "left" \| "right" | - | Position of the icon. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| size | InputSize \| undefined | - | Defines the size of the component. |
| allowEmpty | boolean \| undefined | - | Whether selection can not be cleared. |
| fluid | boolean \| undefined | undefined | Spans 100% width of the container when enabled. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: ToggleButtonChangeEvent | Callback to invoke on value change. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| icon | TemplateRef<ToggleButtonIconTemplateContext> \| undefined | Custom icon template. |
| content | TemplateRef<ToggleButtonContentTemplateContext> \| undefined | Custom content template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| content | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the content's DOM element. |
| icon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the icon's DOM element. |
| label | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the label's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-togglebutton | Class name of the root element |
| p-togglebutton-icon | Class name of the icon element |
| p-togglebutton-icon-left | Class name of the left icon |
| p-togglebutton-icon-right | Class name of the right icon |
| p-togglebutton-label | Class name of the label element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| togglebutton.padding | --p-togglebutton-padding | Padding of root |
| togglebutton.border.radius | --p-togglebutton-border-radius | Border radius of root |
| togglebutton.gap | --p-togglebutton-gap | Gap of root |
| togglebutton.font.size | --p-togglebutton-font-size | Font size of root |
| togglebutton.font.weight | --p-togglebutton-font-weight | Font weight of root |
| togglebutton.disabled.background | --p-togglebutton-disabled-background | Disabled background of root |
| togglebutton.disabled.border.color | --p-togglebutton-disabled-border-color | Disabled border color of root |
| togglebutton.disabled.color | --p-togglebutton-disabled-color | Disabled color of root |
| togglebutton.invalid.border.color | --p-togglebutton-invalid-border-color | Invalid border color of root |
| togglebutton.focus.ring.width | --p-togglebutton-focus-ring-width | Focus ring width of root |
| togglebutton.focus.ring.style | --p-togglebutton-focus-ring-style | Focus ring style of root |
| togglebutton.focus.ring.color | --p-togglebutton-focus-ring-color | Focus ring color of root |
| togglebutton.focus.ring.offset | --p-togglebutton-focus-ring-offset | Focus ring offset of root |
| togglebutton.focus.ring.shadow | --p-togglebutton-focus-ring-shadow | Focus ring shadow of root |
| togglebutton.transition.duration | --p-togglebutton-transition-duration | Transition duration of root |
| togglebutton.sm.font.size | --p-togglebutton-sm-font-size | Sm font size of root |
| togglebutton.sm.padding | --p-togglebutton-sm-padding | Sm padding of root |
| togglebutton.lg.font.size | --p-togglebutton-lg-font-size | Lg font size of root |
| togglebutton.lg.padding | --p-togglebutton-lg-padding | Lg padding of root |
| togglebutton.background | --p-togglebutton-background | Background of root |
| togglebutton.checked.background | --p-togglebutton-checked-background | Checked background of root |
| togglebutton.hover.background | --p-togglebutton-hover-background | Hover background of root |
| togglebutton.border.color | --p-togglebutton-border-color | Border color of root |
| togglebutton.color | --p-togglebutton-color | Color of root |
| togglebutton.hover.color | --p-togglebutton-hover-color | Hover color of root |
| togglebutton.checked.color | --p-togglebutton-checked-color | Checked color of root |
| togglebutton.checked.border.color | --p-togglebutton-checked-border-color | Checked border color of root |
| togglebutton.icon.disabled.color | --p-togglebutton-icon-disabled-color | Disabled color of icon |
| togglebutton.icon.color | --p-togglebutton-icon-color | Color of icon |
| togglebutton.icon.hover.color | --p-togglebutton-icon-hover-color | Hover color of icon |
| togglebutton.icon.checked.color | --p-togglebutton-icon-checked-color | Checked color of icon |
| togglebutton.content.padding | --p-togglebutton-content-padding | Padding of content |
| togglebutton.content.border.radius | --p-togglebutton-content-border-radius | Border radius of content |
| togglebutton.content.checked.shadow | --p-togglebutton-content-checked-shadow | Checked shadow of content |
| togglebutton.content.sm.padding | --p-togglebutton-content-sm-padding | Sm padding of content |
| togglebutton.content.lg.padding | --p-togglebutton-content-lg-padding | Lg padding of content |
| togglebutton.content.checked.background | --p-togglebutton-content-checked-background | Checked background of content |
