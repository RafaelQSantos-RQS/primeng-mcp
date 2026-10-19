---
title: "Angular Chip Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/chip.md"
description: "Chip represents entities using icons, labels and images."
---

# Angular Chip Component

Chip represents entities using icons, labels and images.

## Accessibility

Screen Reader Chip uses the label property as the default aria-label , since any attribute is passed to the root element aria-labelledby or aria-label can be used to override the default behavior. Removable chips have a tabindex and focusable with the tab key. Keyboard Support Key Function backspace Hides removable. enter Hides removable.

## Basic

Displays compact information with an optional remove action.

```typescript
import { Component } from '@angular/core';
import { ChipModule } from 'primeng/chip';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <p-chip label="GitHub" icon="pi pi-github" />
        </div>
    `,
    standalone: true,
    imports: [ChipModule]
})
export class ChipBasicDemo {}
```

## Icon

A font icon next to the label can be displayed with the icon property. Use removable to add a remove action.

```typescript
import { Component } from '@angular/core';
import { ChipModule } from 'primeng/chip';

@Component({
    template: `
        <div class="flex items-center justify-center flex-wrap gap-2">
            <p-chip label="Apple" icon="pi pi-apple" />
            <p-chip label="Facebook" icon="pi pi-facebook" />
            <p-chip label="Google" icon="pi pi-google" />
            <p-chip label="Microsoft" icon="pi pi-microsoft" />
            <p-chip label="GitHub" icon="pi pi-github" [removable]="true" />
        </div>
    `,
    standalone: true,
    imports: [ChipModule]
})
export class ChipIconDemo {}
```

## Image

The image property is used to display an image like an avatar.

```typescript
import { Component } from '@angular/core';
import { ChipModule } from 'primeng/chip';

@Component({
    template: `
        <div class="flex items-center justify-center flex-wrap gap-2">
            <p-chip label="Amy Elsner" image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" />
            <p-chip label="Asiya Javayant" image="https://primefaces.org/cdn/primeng/images/demo/avatar/asiyajavayant.png" />
            <p-chip label="Onyama Limba" image="https://primefaces.org/cdn/primeng/images/demo/avatar/onyamalimba.png" />
            <p-chip label="Xuxue Feng" image="https://primefaces.org/cdn/primeng/images/demo/avatar/xuxuefeng.png" [removable]="true" />
        </div>
    `,
    standalone: true,
    imports: [ChipModule]
})
export class ChipImageDemo {}
```

## preview-doc

```typescript
import { Component, signal } from '@angular/core';
import { ChipModule } from 'primeng/chip';
import { Check } from '@primeicons/angular/check';

@Component({
    template: `
        <div class="max-w-xs mx-auto bg-surface-50/25 dark:bg-surface-900/25 p-4 rounded-lg border border-surface">
            <h2 class="font-bold mb-0.5">What are you interested in?</h2>
            <p class="text-surface-500 mb-3 text-sm">You can select multiple answers.</p>
            <div class="flex items-center justify-start flex-wrap gap-2.5">
                @for (chip of chips; track chip.label; let i = $index) {
                    <p-chip [class]="selectedIndices().has(i) ? 'cursor-pointer' : 'cursor-pointer p-chip-outlined'" (click)="onSelect(i)">
                        @if (selectedIndices().has(i)) {
                            <span class="size-4.5 inline-flex items-center justify-center bg-surface-900 dark:bg-surface-50 text-surface-0 dark:text-surface-900 rounded-full">
                                <svg data-p-icon="check" [size]="10"></svg>
                            </span>
                        } @else {
                            <span>{{ chip.emoji }}</span>
                        }
                        <span class="text-xs">{{ chip.label }}</span>
                    </p-chip>
                }
            </div>
        </div>
    `,
    standalone: true,
    imports: [ChipModule, Check]
})
export class ChipPreviewDemo {
    chips = [
        { label: 'Music', emoji: '🎵' },
        { label: 'Movies', emoji: '🎬' },
        { label: 'Travel', emoji: '✈️' },
        { label: 'Food', emoji: '🍔' },
        { label: 'Sports', emoji: '⚽' },
        { label: 'Gaming', emoji: '🎮' },
        { label: 'Reading', emoji: '📚' },
        { label: 'Photography', emoji: '📸' },
        { label: 'Fitness', emoji: '💪' },
        { label: 'Technology', emoji: '💻' },
        { label: 'Art', emoji: '🎨' },
        { label: 'Nature', emoji: '🌿' },
        { label: 'Shopping', emoji: '🛍️' },
        { label: 'Cooking', emoji: '👨‍🍳' },
        { label: 'Pets', emoji: '🐶' },
        { label: 'Cars', emoji: '🚗' },
        { label: 'Fashion', emoji: '👕' },
        { label: 'Science', emoji: '🔬' }
    ];

    selectedIndices = signal(new Set([0, 4, 7, 10, 15]));

    onSelect(index: number) {
        this.selectedIndices.update((prev) => {
            const next = new Set(prev);

            if (next.has(index)) {
                next.delete(index);
            } else {
                next.add(index);
            }

            return next;
        });
    }
}
```

## Template

Chip also allows displaying custom content inside itself.

```typescript
import { Component } from '@angular/core';
import { ChipModule } from 'primeng/chip';

@Component({
    template: `
        <div class="flex items-center justify-center flex-wrap gap-2">
            <p-chip label="Apple" icon="pi pi-apple" class="bg-neutral-900! dark:bg-neutral-50! **:text-neutral-50! dark:**:text-neutral-900!" />
            <p-chip label="Facebook" icon="pi pi-facebook" class="bg-blue-50! dark:bg-blue-950! **:text-blue-700! dark:**:text-blue-300!" />
            <p-chip label="Google" icon="pi pi-google" class="bg-red-50! dark:bg-red-950! **:text-red-700! dark:**:text-red-300!" />
            <p-chip label="Microsoft" icon="pi pi-microsoft" class="bg-green-50! dark:bg-green-950! **:text-green-700! dark:**:text-green-300!" />
            <p-chip label="GitHub" icon="pi pi-github" [removable]="true" class="bg-purple-50! dark:bg-purple-950! **:text-purple-700! dark:**:text-purple-300!" />
        </div>
    `,
    standalone: true,
    imports: [ChipModule]
})
export class ChipTemplateDemo {}
```

## Chip

Chip represents people using icons, labels and images.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ChipPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| label | string \| undefined | - | Defines the text to display. |
| icon | string \| undefined | - | Defines the icon to display. |
| image | string \| undefined | - | Defines the image to display. |
| alt | string \| undefined | - | Alt attribute of the image. |
| disabled | boolean | - | When present, it specifies that the element should be disabled. |
| removable | boolean | - | Whether to display a remove icon. |
| removeIcon | string \| undefined | - | Icon of the remove element. |
| chipProps | ChipProps \| undefined | - | Used to pass all properties of the chipProps to the Chip component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onRemove | event: MouseEvent | Callback to invoke when a chip is removed. |
| onImageError | event: Event | This event is triggered if an error occurs while loading an image file. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| removeicon | TemplateRef<void> \| undefined | Custom remove icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| image | PassThroughOption<HTMLImageElement, I> | Used to pass attributes to the image's DOM element. |
| icon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the icon's DOM element. |
| label | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the label's DOM element. |
| removeIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the remove icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-chip | Class name of the root element |
| p-chip-image | Class name of the image element |
| p-chip-icon | Class name of the icon element |
| p-chip-label | Class name of the label element |
| p-chip-remove-icon | Class name of the remove icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| chip.border.radius | --p-chip-border-radius | Border radius of root |
| chip.padding.x | --p-chip-padding-x | Padding x of root |
| chip.padding.y | --p-chip-padding-y | Padding y of root |
| chip.gap | --p-chip-gap | Gap of root |
| chip.transition.duration | --p-chip-transition-duration | Transition duration of root |
| chip.background | --p-chip-background | Background of root |
| chip.focus.background | --p-chip-focus-background | Focus background of root |
| chip.color | --p-chip-color | Color of root |
| chip.image.width | --p-chip-image-width | Width of image |
| chip.image.height | --p-chip-image-height | Height of image |
| chip.icon.size | --p-chip-icon-size | Size of icon |
| chip.icon.color | --p-chip-icon-color | Color of icon |
| chip.label.font.weight | --p-chip-label-font-weight | Font weight of label |
| chip.label.font.size | --p-chip-label-font-size | Font size of label |
| chip.remove.icon.size | --p-chip-remove-icon-size | Size of remove icon |
| chip.remove.icon.focus.ring.width | --p-chip-remove-icon-focus-ring-width | Focus ring width of remove icon |
| chip.remove.icon.focus.ring.style | --p-chip-remove-icon-focus-ring-style | Focus ring style of remove icon |
| chip.remove.icon.focus.ring.color | --p-chip-remove-icon-focus-ring-color | Focus ring color of remove icon |
| chip.remove.icon.focus.ring.offset | --p-chip-remove-icon-focus-ring-offset | Focus ring offset of remove icon |
| chip.remove.icon.focus.ring.shadow | --p-chip-remove-icon-focus-ring-shadow | Focus ring shadow of remove icon |
| chip.remove.icon.color | --p-chip-remove-icon-color | Color of remove icon |
