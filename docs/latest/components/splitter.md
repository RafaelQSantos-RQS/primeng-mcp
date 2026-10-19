---
title: "Angular Splitter Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/splitter.md"
description: "Splitter is utilized to separate and resize panels."
---

# Angular Splitter Component

Splitter is utilized to separate and resize panels.

## Accessibility

Screen Reader Splitter bar defines separator as the role with aria-orientation set to either horizontal or vertical. Keyboard Support Key Function tab Moves focus through the splitter bar. down arrow Moves a vertical splitter down. up arrow Moves a vertical splitter up. left arrow Moves a vertical splitter to the left. right arrow Moves a vertical splitter to the right.

## Advanced

A real-world layout combining a collapsible navigation pane, a custom gutter handle and a viewport that adapts to the current sizes. The pane switches to a compact list once its width drops below 28%.

```typescript
import { Component, signal } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter
            [panelSizes]="[35, 65]"
            class="h-150!"
            [pt]="{
                gutterHandle: { class: 'absolute! inset-y-0! h-full! rounded-none w-2! left-1/2! -translate-x-1/2! bg-surface-200! dark:bg-surface-700! hover:opacity-100! data-[resizing]:opacity-100! opacity-0! transition-opacity! delay-100!' },
                gutter: { class: 'relative!' }
            }"
            (onResize)="onResize($event)"
            (onResizeEnd)="onResize($event)"
        >
            <p-splitter-panel [minSize]="20" collapsible>
                <ul class="list-none m-0 p-0 w-full">
                    @for (mail of mails; track $index) {
                        <li
                            class="px-4 py-3 cursor-pointer border-b border-surface-100 dark:border-surface-800 transition-colors"
                            [class]="selected() === $index ? 'bg-primary-50 dark:bg-primary-950' : 'hover:bg-surface-50 dark:hover:bg-surface-800'"
                            (click)="selected.set($index)"
                        >
                            <div class="flex items-center justify-between gap-2">
                                <span class="font-semibold text-surface-900 dark:text-surface-0 text-sm truncate">{{ mail.name }}</span>
                                <span class="text-surface-400 text-xs shrink-0">{{ mail.time }}</span>
                            </div>
                            @if (!isCompact()) {
                                <div class="text-surface-700 dark:text-surface-200 text-sm font-medium mt-1 mb-1">{{ mail.title }}</div>
                                <div class="text-surface-400 dark:text-surface-500 text-xs line-clamp-1">{{ mail.description }}</div>
                            }
                        </li>
                    }
                </ul>
            </p-splitter-panel>
            <p-splitter-panel [minSize]="30">
                <div class="p-6">
                    <div class="flex items-start justify-between gap-4 mb-2">
                        <h2 class="text-lg font-semibold text-surface-900 dark:text-surface-0 m-0!">{{ mails[selected()].title }}</h2>
                        <span class="text-surface-400 text-xs shrink-0 pt-1">{{ mails[selected()].time }}</span>
                    </div>
                    <span class="text-surface-400 text-sm">{{ mails[selected()].name }}</span>
                    <p class="text-surface-600 dark:text-surface-300 text-sm leading-relaxed mt-4 m-0!">{{ mails[selected()].description }}</p>
                </div>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterAdvancedDemo {
    mails = mails;

    selected = signal(0);

    sizes = signal<number[]>([35, 65]);

    isCompact = computed(() => this.sizes()[0] < 28);

    onResize(event: SplitterResizeEvent) {
        this.sizes.set(event.sizes as number[]);
    }
}
```

## Basic

Divides a layout into resizable panels with a draggable divider.

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter class="h-60! max-w-lg mx-auto">
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
            </p-splitter-panel>
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterBasicDemo {}
```

## Custom

The default appearance is fully overridable through Tailwind utilities on the panels and the pt API on the gutter. Below, panels become colored badge containers and the gutter is rendered as a thin rounded bar.

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter
            class="h-60! max-w-lg mx-auto border-none! gap-0.5! bg-transparent!"
            [pt]="{
                gutter: { class: 'w-1.5! rounded-xs!' }
            }"
        >
            <p-splitter-panel class="flex items-center justify-center border border-blue-500 bg-blue-500/10 rounded-sm text-blue-500">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-current/20">1</span>
            </p-splitter-panel>
            <p-splitter-panel class="flex items-center justify-center border border-purple-500 bg-purple-500/10 rounded-sm text-purple-500">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-current/20">2</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterCustomDemo {}
```

## Min Max Size

Each panel can declare a minSize and a maxSize in percentages. The splitter ensures the panel cannot shrink below minSize nor grow beyond maxSize during a resize.

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <div class="flex flex-col items-center gap-8 max-w-lg mx-auto">
            <p-splitter class="w-full h-32!">
                <p-splitter-panel [minSize]="20" [maxSize]="50" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
                </p-splitter-panel>
                <p-splitter-panel class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
                </p-splitter-panel>
            </p-splitter>
            <p-splitter class="w-full h-32!">
                <p-splitter-panel [minSize]="10" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
                </p-splitter-panel>
                <p-splitter-panel [minSize]="10" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
                </p-splitter-panel>
                <p-splitter-panel [minSize]="10" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">3</span>
                </p-splitter-panel>
            </p-splitter>
        </div>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterMinmaxDemo {}
```

## Nested

Splitters can be nested by placing another Splitter inside a SplitterPanel.

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter [panelSizes]="[25, 75]" class="h-78! max-w-lg mx-auto">
            <p-splitter-panel [minSize]="10" class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
            </p-splitter-panel>
            <p-splitter-panel>
                <p-splitter layout="vertical" [panelSizes]="[50, 50]" class="h-full">
                    <p-splitter-panel class="flex items-center justify-center">
                        <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
                    </p-splitter-panel>
                    <p-splitter-panel>
                        <p-splitter [panelSizes]="[20, 80]" class="h-full">
                            <p-splitter-panel class="flex items-center justify-center">
                                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">3</span>
                            </p-splitter-panel>
                            <p-splitter-panel class="flex items-center justify-center">
                                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">4</span>
                            </p-splitter-panel>
                        </p-splitter>
                    </p-splitter-panel>
                </p-splitter>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterNestedDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter class="h-60! max-w-lg mx-auto">
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
            </p-splitter-panel>
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterPreviewDemo {}
```

## resizeevents-doc

Splitters can trigger events during the resize process, allowing you to handle custom logic when a panel is being resized.

```typescript
import { Component, signal } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4 max-w-lg mx-auto">
            <div class="flex flex-col items-center gap-1 w-full text-sm font-mono text-surface-500">
                <div>onResizeStart: [{{ resizeStart() }}]</div>
                <div>onResizeEnd: [{{ resizeEnd() }}]</div>
            </div>
            <p-splitter class="w-full h-60!" (onResizeStart)="onResizeStartEvent($event)" (onResizeEnd)="onResizeEndEvent($event)">
                <p-splitter-panel [minSize]="15" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
                </p-splitter-panel>
                <p-splitter-panel [minSize]="25" class="flex items-center justify-center">
                    <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
                </p-splitter-panel>
            </p-splitter>
        </div>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterResizeeventsDemo {
    resizeStart = signal<string>('50.0%, 50.0%');

    resizeEnd = signal<string>('50.0%, 50.0%');

    onResizeStartEvent(event: any) {
        this.resizeStart.set(event.sizes.map((s: number) => s.toFixed(1) + '%').join(', '));
    }

    onResizeEndEvent(event: any) {
        this.resizeEnd.set(event.sizes.map((s: number) => s.toFixed(1) + '%').join(', '));
    }
}
```

## Size

Initial dimensions are percentage based and provided through panelSizes on the Splitter. Per-panel constraints are configured with minSize and maxSize on each SplitterPanel.

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter [panelSizes]="[25, 75]" class="h-60! max-w-lg mx-auto">
            <p-splitter-panel [minSize]="10" class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
            </p-splitter-panel>
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterSizeDemo {}
```

## Stateful

Panel sizes can be persisted using the stateKey and stateStorage properties. This example saves sizes to localStorage so they are restored on page reload.

```typescript
import { Component, inject, signal, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter [panelSizes]="[30, 70]" stateKey="splitter-demo-sizes" stateStorage="local" class="h-60! max-w-lg mx-auto" (onResizeEnd)="onResizeEnd($event)">
            <p-splitter-panel [minSize]="10" class="flex items-center justify-center gap-2">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
                <span class="text-sm text-muted-color tabular-nums">({{ sizes()[0].toFixed(1) }}%)</span>
            </p-splitter-panel>
            <p-splitter-panel [minSize]="10" class="flex items-center justify-center gap-2">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
                <span class="text-sm text-muted-color tabular-nums">({{ sizes()[1].toFixed(1) }}%)</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterStatefulDemo {
    private platformId = inject(PLATFORM_ID);

    sizes = signal<number[]>(this.getInitialSizes());

    private getInitialSizes(): number[] {
        if (isPlatformBrowser(this.platformId)) {
            const stored = localStorage.getItem('splitter-demo-sizes');

            if (stored) {
                try {
                    const parsed = JSON.parse(stored);

                    if (Array.isArray(parsed) && parsed.length === 2 && parsed.every((v: unknown) => typeof v === 'number' && isFinite(v))) {
                        return parsed;
                    }
                } catch {
                    // Stored value is not valid JSON; fall through to the default sizes.
                }
            }
        }

        return [30, 70];
    }

    onResizeEnd(event: any) {
        this.sizes.set(event.sizes);
    }
}
```

## Vertical

Panels are displayed as stacked by setting the layout to vertical .

```typescript
import { Component } from '@angular/core';
import { SplitterModule } from 'primeng/splitter';

@Component({
    template: `
        <p-splitter class="h-60! max-w-lg mx-auto" layout="vertical">
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">1</span>
            </p-splitter-panel>
            <p-splitter-panel class="flex items-center justify-center">
                <span class="px-2.5 py-1 rounded-md font-mono font-bold bg-surface-100 dark:bg-surface-800 text-color">2</span>
            </p-splitter-panel>
        </p-splitter>
    `,
    standalone: true,
    imports: [SplitterModule]
})
export class SplitterVerticalDemo {}
```

## Splitter

Splitter is utilized to separate and resize panels.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SplitterPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| styleClass | string \| undefined | - | Style class of the component. |
| panelStyleClass | string \| undefined | - | Style class of the panel. |
| panelStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the panel. |
| stateStorage | "session" \| "local" | - | Defines where a stateful splitter keeps its state, valid values are 'session' for sessionStorage and 'local' for localStorage. |
| stateKey | string \| null | - | Storage identifier of a stateful Splitter. |
| layout | "horizontal" \| "vertical" | - | Orientation of the panels. Valid values are 'horizontal' and 'vertical'. |
| gutterSize | number | - | Size of the divider in pixels. |
| step | number | - | Step factor to increment/decrement the size of the panels while pressing the arrow keys. |
| minSizes | number[] | - | Minimum size of the elements relative to 100%. Used when panels are defined via  `#panel`  templates; when SplitterPanel components are used, the per-panel  `minSize`  input takes precedence. |
| panelSizes | number[] | - | Size of the elements relative to 100%. Used when panels are defined via  `#panel`  templates; when SplitterPanel components are used, the per-panel  `size`  input takes precedence. |
| disabled | boolean | false | When present, it disables the component and prevents resizing. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onResizeEnd | event: SplitterResizeEndEvent | Callback to invoke when resize ends. |
| onResizeStart | event: SplitterResizeStartEvent | Callback to invoke when resize starts. |
| onResize | event: SplitterResizeEvent | Callback to invoke continuously while a gutter is being dragged. |
| onCollapse | event: SplitterCollapseEvent | Callback to invoke when a collapsible panel collapses or expands. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| panel | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the panel's DOM element. |
| gutter | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the gutter's DOM element. |
| gutterHandle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the gutter handle's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-splitter | Class name of the root element |
| p-splitter-panel | Class name of the panel element |
| p-splitter-gutter | Class name of the gutter element |
| p-splitter-gutter-handle | Class name of the handle element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| splitter.background | --p-splitter-background | Background of root |
| splitter.border.color | --p-splitter-border-color | Border color of root |
| splitter.color | --p-splitter-color | Color of root |
| splitter.transition.duration | --p-splitter-transition-duration | Transition duration of root |
| splitter.gutter.background | --p-splitter-gutter-background | Background of gutter |
| splitter.handle.size | --p-splitter-handle-size | Size of handle |
| splitter.handle.background | --p-splitter-handle-background | Background of handle |
| splitter.handle.border.radius | --p-splitter-handle-border-radius | Border radius of handle |
| splitter.handle.focus.ring.width | --p-splitter-handle-focus-ring-width | Focus ring width of handle |
| splitter.handle.focus.ring.style | --p-splitter-handle-focus-ring-style | Focus ring style of handle |
| splitter.handle.focus.ring.color | --p-splitter-handle-focus-ring-color | Focus ring color of handle |
| splitter.handle.focus.ring.offset | --p-splitter-handle-focus-ring-offset | Focus ring offset of handle |
| splitter.handle.focus.ring.shadow | --p-splitter-handle-focus-ring-shadow | Focus ring shadow of handle |
