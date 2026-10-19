---
title: "Angular Rating Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/rating.md"
description: "Rating component is a star based selection input."
---

# Angular Rating Component

Rating component is a star based selection input.

## Accessibility

Screen Reader Rating component internally uses radio buttons that are only visible to screen readers. The value to read for item is retrieved from the locale API via star and stars of the aria property.

## Basic

Two-way value binding is defined using ngModel .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" [allowHalf]="true" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingBasicDemo {
    value: number = 3.5;
}
```

## Controlled

Rating value can be controlled programmatically.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex flex-col items-center justify-center gap-6">
            <p-rating [(ngModel)]="value" [allowHalf]="true" />
            <div class="flex items-center gap-2">
                <button pButton (click)="value = 2.5" severity="secondary" variant="outlined" size="small">2.5 Star</button>
                <button pButton (click)="value = 3" severity="secondary" variant="outlined" size="small">3 Star</button>
                <button pButton (click)="value = 3.5" severity="secondary" variant="outlined" size="small">3.5 Star</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [RatingModule, ButtonModule, FormsModule]
})
export class RatingControlledDemo {
    value: number = 4;
}
```

## Disabled

When disabled is present, a visual hint is applied to indicate that the Knob cannot be interacted with.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" [disabled]="true" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingDisabledDemo {
    value: number = 3;
}
```

## Emoji

Custom content can be used for rating icons using templates.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" [allowHalf]="false" [stars]="5">
                <ng-template #onicon let-index>
                    <span class="text-4xl select-none transition-all">{{ emojis[index - 1] }}</span>
                </ng-template>
                <ng-template #officon let-index>
                    <span class="text-4xl select-none transition-all grayscale">{{ emojis[index - 1] }}</span>
                </ng-template>
            </p-rating>
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingEmojiDemo {
    value: number = 3;

    emojis: string[] = ['😡', '😕', '😐', '🙂', '😍'];
}
```

## Half Stars

Enable allowHalf property to select half stars.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex flex-col gap-6 items-center justify-center">
            <div class="flex flex-col gap-2">
                <label pLabel>Full Star</label>
                <p-rating [(ngModel)]="value1" />
            </div>
            <div class="flex flex-col gap-2">
                <label pLabel>Half Star</label>
                <p-rating [(ngModel)]="value2" [allowHalf]="true" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [RatingModule, LabelModule, FormsModule]
})
export class RatingHalfDemo {
    value1: number = 3;

    value2: number = 3;
}
```

## Number of Stars

Number of stars to display is defined with stars property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" [stars]="10" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingNumberOfStarsDemo {
    value: number = 5;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="max-w-sm mx-auto">
            <div class="flex items-start gap-3">
                <div class="relative shrink-0">
                    <p-avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" size="large" />
                    <span class="absolute bottom-0 right-0 size-3 rounded-full bg-green-500 ring-2 ring-surface-0 dark:ring-surface-900"></span>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-baseline gap-2">
                        <span class="text-color font-semibold text-sm">Amy Elsner</span>
                        <span class="text-xs text-muted-color">Support · 2m</span>
                    </div>
                    <div class="mt-1 rounded-2xl rounded-tl-sm bg-surface-100 dark:bg-surface-800 px-4 py-2.5 text-color">Glad I could help! How would you rate this conversation?</div>
                    <div class="mt-4">
                        <p-rating [(ngModel)]="value" />
                    </div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, RatingModule, FormsModule]
})
export class RatingPreviewDemo {
    value: number = 4;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { RatingModule } from 'primeng/rating';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 w-40">
                <div class="flex flex-col items-center gap-2">
                    <p-rating formControlName="ratingValue" [invalid]="isInvalid('ratingValue')" />
                    @if (isInvalid('ratingValue')) {
                        <p-message severity="error" size="small" variant="simple">Value is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RatingModule, ToastModule, ButtonModule, ReactiveFormsModule],
    providers: [MessageService]
})
export class RatingReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            ratingValue: [undefined, Validators.required]
        });
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }

    isInvalid(controlName: string) {
        const control = this.exampleForm.get(controlName);

        return control?.invalid && (control.touched || this.formSubmitted);
    }
}
```

## ReadOnly

When readonly present, value cannot be edited.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" [readonly]="true" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingReadonlyDemo {
    value: number = 3;
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { RatingModule } from 'primeng/rating';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 w-40">
                <div class="flex flex-col items-center gap-2">
                    <p-rating [formField]="exampleForm.ratingValue" />
                    @if (exampleForm.ratingValue().touched() && exampleForm.ratingValue().invalid()) {
                        @for (error of exampleForm.ratingValue().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RatingModule, ToastModule, ButtonModule, FormField]
})
export class RatingSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal<{ ratingValue: number | null }>({ ratingValue: null });

    exampleForm = form(this.model, (path) => {
        required(path.ratingValue, { when: ({ state }) => state.touched(), message: 'Rating is required.' });
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

Templating allows customizing the content where the icon instance is available as the implicit variable.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex flex-col items-center justify-center gap-6">
            <p-rating [(ngModel)]="value1" [allowHalf]="true">
                <ng-template #onicon>
                    <span class="text-surface-950 dark:text-surface-0 font-medium text-4xl select-none">A</span>
                </ng-template>
                <ng-template #officon>
                    <span class="text-surface-300 dark:text-surface-700 font-medium text-4xl select-none">A</span>
                </ng-template>
            </p-rating>
            <p-rating [(ngModel)]="value2">
                <ng-template #onicon>
                    <span class="size-7">
                        <img src="https://primefaces.org/cdn/primevue/images/rating/custom-onicon.png" class="size-7" />
                    </span>
                </ng-template>
                <ng-template #officon>
                    <span class="size-7">
                        <img src="https://primefaces.org/cdn/primevue/images/rating/custom-officon.png" class="size-7" />
                    </span>
                </ng-template>
            </p-rating>
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingTemplateDemo {
    value1: number = 3;

    value2: number = 3;
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { RatingModule } from 'primeng/rating';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 w-40">
                <div class="flex flex-col items-center gap-2">
                    <p-rating #ratingValue="ngModel" [(ngModel)]="value" required name="ratingValue" [invalid]="ratingValue.invalid && (ratingValue.touched || exampleForm.submitted)" />
                    @if (ratingValue.invalid && (ratingValue.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Value is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, RatingModule, ToastModule, ButtonModule, FormsModule],
    providers: [MessageService]
})
export class RatingTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: any;

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Vertical

Rating can be displayed vertically by setting the orientation property to vertical .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" orientation="vertical" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingVerticalDemo {
    value: number = 3;
}
```

## withoutcancel-doc

A cancel icon is displayed to reset the value by default, set cancel as false to remove this option.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RatingModule } from 'primeng/rating';

@Component({
    template: `
        <div class="flex justify-center">
            <p-rating [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [RatingModule, FormsModule]
})
export class RatingWithoutCancelDemo {
    value!: number;
}
```

## Rating

Rating is an extension to standard radio button element with theming.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, RatingPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| readonly | boolean | - | When present, changing the value is not possible. |
| stars | number | - | Number of stars. |
| iconOnClass | string \| undefined | - | Style class of the on icon. |
| iconOnStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the on icon. |
| iconOffClass | string \| undefined | - | Style class of the off icon. |
| iconOffStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the off icon. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| allowHalf | boolean | - | When present, selecting half stars is allowed. |
| orientation | "vertical" \| "horizontal" | - | Orientation of the component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onRate | event: RatingRateEvent | Emitted on value change. |
| onFocus | event: FocusEvent | Emitted when the rating receives focus. |
| onBlur | event: FocusEvent | Emitted when the rating loses focus. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| onicon | TemplateRef<RatingIconTemplateContext> \| undefined | Custom on icon template. |
| officon | TemplateRef<RatingIconTemplateContext> \| undefined | Custom off icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| option | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the option's DOM element. |
| onIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the on icon's DOM element. |
| offIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the off icon's DOM element. |
| hiddenOptionInputContainer | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden option input container's DOM element. |
| hiddenOptionInput | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the hidden option input's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-rating | Class name of the root element |
| p-rating-option | Class name of the option element |
| p-rating-on-icon | Class name of the on icon element |
| p-rating-off-icon | Class name of the off icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| rating.gap | --p-rating-gap | Gap of root |
| rating.transition.duration | --p-rating-transition-duration | Transition duration of root |
| rating.focus.ring.width | --p-rating-focus-ring-width | Focus ring width of root |
| rating.focus.ring.style | --p-rating-focus-ring-style | Focus ring style of root |
| rating.focus.ring.color | --p-rating-focus-ring-color | Focus ring color of root |
| rating.focus.ring.offset | --p-rating-focus-ring-offset | Focus ring offset of root |
| rating.focus.ring.shadow | --p-rating-focus-ring-shadow | Focus ring shadow of root |
| rating.icon.size | --p-rating-icon-size | Size of icon |
| rating.icon.color | --p-rating-icon-color | Color of icon |
| rating.icon.hover.color | --p-rating-icon-hover-color | Hover color of icon |
| rating.icon.active.color | --p-rating-icon-active-color | Active color of icon |
