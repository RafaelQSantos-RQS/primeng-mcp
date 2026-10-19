---
title: "Angular InputTags Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inputtags.md"
description: "InputTags is used to enter multiple tags."
---

# Angular InputTags Component

InputTags is used to enter multiple tags.

## Accessibility

Screen Reader The input tags component uses a listbox role with aria-orientation set to horizontal. Each tag has the option role with aria-label , aria-selected , aria-setsize and aria-posinset attributes. Keyboard Support Key Function Enter Adds the typed value as a tag. Backspace Removes the last tag when the input is empty. Left Arrow Moves focus to the previous tag. Right Arrow Moves focus to the next tag.

## Allow Duplicate

By default, duplicate values are not allowed. Set allowDuplicate to enable adding the same value multiple times.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" [allowDuplicate]="true" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsAllowDuplicateDemo {
    value: string[] = ['a', 'A', 'a'];
}
```

## Basic

InputTags is used to enter multiple tags. Press Enter to add a tag and Backspace to remove the last one.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsBasicDemo {
    value: string[] = ['Angular'];
}
```

## Delimiter

A custom delimiter like a comma can be used in addition to the Enter key to add tags. Enable addOnPaste to split pasted text using the delimiter.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" delimiter="," [addOnPaste]="true" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsDelimiterDemo {
    value: string[] = [];
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" [disabled]="true" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsDisabledDemo {
    value: string[] = ['Angular'];
}
```

## Events

Use onAdd and onRemove callbacks to listen for tag changes.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" (onAdd)="onAddItem($event)" (onRemove)="onRemoveItem($event)" placeholder="Add tags and watch the event log..." fluid />
        <div class="mt-4 text-sm">
            @for (log of logs; track $index) {
                <p>{{ log }}</p>
            }
        </div>
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsEventsDemo {
    value: string[] = ['Angular'];

    logs: string[] = [];

    onAddItem(event: any) {
        this.logs = [`Added: "${event.value}"`, ...this.logs.slice(0, 4)];
    }

    onRemoveItem(event: any) {
        this.logs = [`Removed: "${event.value}" at index ${event.index}`, ...this.logs.slice(0, 4)];
    }
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" variant="filled" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsFilledDemo {
    value: string[] = ['Angular'];
}
```

## Float Label

A floating label appears on top of the input field when focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FloatLabelModule } from 'primeng/floatlabel';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-floatlabel>
            <p-inputtags [(ngModel)]="value" inputId="tags" fluid />
            <label for="tags">Technologies</label>
        </p-floatlabel>
    `,
    standalone: true,
    imports: [FloatLabelModule, InputTagsModule, FormsModule]
})
export class InputTagsFloatLabelDemo {
    value: string[] = [];
}
```

## Ifta Label

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-iftalabel>
            <p-inputtags [(ngModel)]="tags" inputId="it_tags" fluid />
            <label for="it_tags">Technologies</label>
        </p-iftalabel>
    `,
    standalone: true,
    imports: [IftaLabelModule, InputTagsModule, FormsModule]
})
export class InputTagsIftaLabelDemo {
    tags: string[] = [];
}
```

## Invalid

The invalid property is used to indicate an invalid state.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" [invalid]="value.length === 0" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsInvalidDemo {
    value: string[] = [];
}
```

## Max

The maximum number of tags is limited using the max property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" [max]="5" fluid />
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsMaxDemo {
    value: string[] = ['Angular', 'React'];
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';

@Component({
    template: `
        <div class="max-w-3xs mx-auto flex flex-col">
            <label class="mb-2 text-sm font-medium">Frameworks</label>
            <p-inputtags [(ngModel)]="value" />
        </div>
    `,
    standalone: true,
    imports: [InputTagsModule, FormsModule]
})
export class InputTagsPreviewDemo {
    value: string[] = ['React', 'JavaScript', 'Vue', 'Angular'];
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { InputTagsModule } from 'primeng/inputtags';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-inputtags formControlName="tags" [invalid]="isInvalid('tags')" />
                    @if (isInvalid('tags')) {
                        <p-message severity="error" size="small" variant="simple">At least one tag is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, InputTagsModule, ButtonModule, ReactiveFormsModule]
})
export class InputTagsReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            tags: [null, Validators.required]
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

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, submit, validate } from '@angular/forms/signals';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { InputTagsModule } from 'primeng/inputtags';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-inputtags [formField]="exampleForm.tags" />
                    @if (exampleForm.tags().touched() && exampleForm.tags().invalid()) {
                        @for (error of exampleForm.tags().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, InputTagsModule, ButtonModule, FormField]
})
export class InputTagsSignalFormsDemo {
    messageService = inject(MessageService);

    model = signal<{ tags: string[] }>({ tags: [] });

    exampleForm = form(this.model, (path) => {
        validate(path.tags, ({ value, state }) => {
            if (!state.touched()) return null;

            return value().length > 0 ? null : { kind: 'minLength', message: 'At least one tag is required.' };
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

Custom content can be displayed for each tag using the item template.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TagModule } from 'primeng/tag';
import { InputTagsModule } from 'primeng/inputtags';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <p-inputtags [(ngModel)]="value" fluid>
            <ng-template #item let-item let-i="index" let-onRemove="onRemove">
                <p-tag [rounded]="true">
                    {{ item }}
                    <svg data-p-icon="times" class="cursor-pointer ml-2" style="width: 12px; height: 12px" (click)="onRemove($event, i)"></svg>
                </p-tag>
            </ng-template>
        </p-inputtags>
    `,
    standalone: true,
    imports: [TagModule, InputTagsModule, FormsModule, Times]
})
export class InputTagsTemplateDemo {
    value: string[] = ['JavaScript', 'TypeScript'];
}
```

## templatedrivenforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { InputTagsModule } from 'primeng/inputtags';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-inputtags name="tags" [(ngModel)]="value" #tags="ngModel" [invalid]="tags.invalid && (tags.touched || exampleForm.submitted)" required />
                    @if (tags.invalid && (tags.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">At least one tag is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [MessageModule, ToastModule, InputTagsModule, ButtonModule, FormsModule]
})
export class InputTagsTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    value: string[] = [];

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Typeahead

When typeahead is enabled, a dropdown with suggestions is displayed as the user types. Use the completeMethod event to provide search results.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTagsModule } from 'primeng/inputtags';
import { LabelModule } from 'primeng/label';
import { InputTagsCompleteEvent } from 'primeng/types/inputtags';

@Component({
    template: `
        <div class="flex justify-center">
            <p-inputtags
                [(ngModel)]="values"
                [typeahead]="true"
                [group]="true"
                [allowDuplicate]="true"
                [suggestions]="filteredItems"
                (completeMethod)="search($event)"
                optionLabel="label"
                optionGroupLabel="label"
                optionGroupChildren="items"
                placeholder="Search technologies..."
                class="w-full md:w-56"
            />
        </div>
    `,
    standalone: true,
    imports: [InputTagsModule, LabelModule, FormsModule]
})
export class InputTagsTypeaheadDemo {
    values: any[] = [];

    items = [
        {
            label: 'Frontend',
            items: [{ label: 'React' }, { label: 'Vue' }, { label: 'Angular' }, { label: 'Svelte' }, { label: 'Next.js' }, { label: 'Nuxt' }]
        },
        {
            label: 'Backend',
            items: [{ label: 'Node.js' }, { label: 'Python' }, { label: 'Java' }, { label: 'Go' }, { label: 'Rust' }, { label: 'Ruby' }]
        }
    ];

    filteredItems: any[] = [];

    search(event: InputTagsCompleteEvent) {
        const query = event.query.toLowerCase();

        this.filteredItems = this.items
            .map((group) => ({
                ...group,
                items: group.items.filter((item) => item.label.toLowerCase().includes(query))
            }))
            .filter((group) => group.items.length > 0);
    }
}
```

## Input Tags

InputTags is used to enter multiple tags with optional typeahead suggestions.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InputTagsPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
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
| min | any | undefined | The value must be greater than or equal to the value. |
| step | number \| null \| undefined | undefined | Unless the step is set to the any literal, the value must be min + an integral multiple of the step. |
| minlength | number \| null \| undefined | undefined | The number of characters (code points) must not be less than the value of the attribute, if non-empty. |
| maxlength | number \| null \| undefined | undefined | The number of characters (code points) must not exceed the value of the attribute. |
| placeholder | string \| undefined | - | Advisory information to display on input. |
| max | any | - | Maximum number of tags allowed. |
| delimiter | string \| RegExp \| undefined | - | Delimiter character or regex to split input into tags. |
| allowDuplicate | boolean | false | Whether to allow duplicate tags. |
| addOnBlur | boolean | false | Whether to add a tag when the input loses focus. |
| addOnTab | boolean | false | Whether to add a tag on tab key press. |
| addOnPaste | boolean | false | Whether to add tags on paste event. |
| tabindex | number \| undefined | - | Index of the element in tabbing order. |
| typeahead | boolean | false | Whether typeahead mode is active. |
| suggestions | any[] | - | An array of suggestions to display in the dropdown. |
| optionLabel | string \| ((item: any) => string) \| undefined | - | Property name or getter function for option label. |
| optionValue | string \| ((item: any) => any) \| undefined | - | Property name or getter function for option value. |
| optionDisabled | string \| ((item: any) => boolean) \| undefined | - | Property name or getter function for disabled state. |
| optionGroupLabel | string | 'label' | Field name for option group label. |
| optionGroupChildren | string | 'items' | Field name for option group children. |
| group | boolean | false | Whether to display options as grouped. |
| delay | number | 300 | Delay in ms between keystrokes before triggering search. |
| minQueryLength | number | 1 | Minimum number of characters to initiate a search. |
| completeOnFocus | boolean | false | Whether to trigger search when input receives focus. |
| scrollHeight | string | '200px' | Maximum height of the suggestions panel. |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'body' | Target element to attach the overlay. |
| overlayOptions | OverlayOptions \| undefined | - | Configuration for the overlay element. |
| motionOptions | MotionOptions \| undefined | - | Animation options for the overlay. |
| panelStyle | { [klass: string]: any } \| undefined | - | Inline style of the overlay panel. |
| panelStyleClass | string \| undefined | - | CSS class of the overlay panel. |
| inputStyle | { [klass: string]: any } \| undefined | - | Inline style of the input field. |
| inputStyleClass | string \| undefined | - | CSS class of the input field. |
| autoOptionFocus | boolean | false | Whether to auto-focus first option when overlay shows. |
| focusOnHover | boolean | true | Whether hover focuses an option. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| completeMethod | event: InputTagsCompleteEvent | Callback to invoke to search for suggestions. |
| onSelect | event: InputTagsSelectEvent | Callback to invoke when a suggestion is selected. |
| onShow | value: void | Callback to invoke when the overlay is shown. |
| onHide | value: void | Callback to invoke when the overlay is hidden. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| pcChip | PassThroughOption<ChipPassThrough, I> | Used to pass attributes to the Chip component. |
| chipIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the chip icon's DOM element. |
| pcAutoComplete | PassThroughOption<AutoCompletePassThrough, I> | Used to pass attributes to the AutoComplete component. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inputtags |  |
| p-inputtags-item |  |
| p-inputtags-input |  |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| inputtags.background | --p-inputtags-background | Background of root |
| inputtags.disabled.background | --p-inputtags-disabled-background | Disabled background of root |
| inputtags.filled.background | --p-inputtags-filled-background | Filled background of root |
| inputtags.filled.hover.background | --p-inputtags-filled-hover-background | Filled hover background of root |
| inputtags.filled.focus.background | --p-inputtags-filled-focus-background | Filled focus background of root |
| inputtags.border.color | --p-inputtags-border-color | Border color of root |
| inputtags.hover.border.color | --p-inputtags-hover-border-color | Hover border color of root |
| inputtags.focus.border.color | --p-inputtags-focus-border-color | Focus border color of root |
| inputtags.invalid.border.color | --p-inputtags-invalid-border-color | Invalid border color of root |
| inputtags.color | --p-inputtags-color | Color of root |
| inputtags.disabled.color | --p-inputtags-disabled-color | Disabled color of root |
| inputtags.shadow | --p-inputtags-shadow | Shadow of root |
| inputtags.padding.x | --p-inputtags-padding-x | Padding x of root |
| inputtags.padding.y | --p-inputtags-padding-y | Padding y of root |
| inputtags.border.radius | --p-inputtags-border-radius | Border radius of root |
| inputtags.focus.ring.width | --p-inputtags-focus-ring-width | Focus ring width of root |
| inputtags.focus.ring.style | --p-inputtags-focus-ring-style | Focus ring style of root |
| inputtags.focus.ring.color | --p-inputtags-focus-ring-color | Focus ring color of root |
| inputtags.focus.ring.offset | --p-inputtags-focus-ring-offset | Focus ring offset of root |
| inputtags.focus.ring.shadow | --p-inputtags-focus-ring-shadow | Focus ring shadow of root |
| inputtags.transition.duration | --p-inputtags-transition-duration | Transition duration of root |
| inputtags.gap | --p-inputtags-gap | Gap of root |
| inputtags.item.border.radius | --p-inputtags-item-border-radius | Border radius of item |
