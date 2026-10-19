---
title: "Angular Skeleton Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/skeleton.md"
description: "Skeleton is a placeholder to display instead of the actual content."
---

# Angular Skeleton Component

Skeleton is a placeholder to display instead of the actual content.

## Accessibility

Screen Reader Skeleton uses aria-hidden as "true" so that it gets ignored by screen readers, any valid attribute is passed to the root element so you may customize it further if required. If multiple skeletons are grouped inside a container, you may use aria-busy on the container element as well to indicate the loading process.

## Basic

Placeholder shapes that mimic content layout during loading.

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="flex gap-4 max-w-xs mx-auto">
            <p-skeleton shape="circle" size="2.5rem" />
            <div class="flex-1 flex flex-col gap-1.5 py-0.5">
                <p-skeleton width="100%" borderRadius="4px" class="h-auto! flex-1" />
                <p-skeleton width="90%" borderRadius="4px" class="h-auto! flex-1" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonBasicDemo {}
```

## Card

Sample Card implementation using different Skeleton components and Tailwind CSS utilities.

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="max-w-sm w-full space-y-4">
                <div class="flex items-start gap-4">
                    <p-skeleton shape="circle" size="3.5rem" />
                    <div class="flex-1">
                        <div class="flex items-center justify-between">
                            <p-skeleton width="40%" height="1.5rem" />
                        </div>
                        <div class="space-y-1.5 mt-3">
                            <p-skeleton width="100%" borderRadius="4px" />
                            <p-skeleton width="90%" borderRadius="4px" />
                        </div>
                        <p-skeleton class="mt-4 aspect-video w-full h-auto!" />
                        <div class="flex items-center gap-4 mt-4">
                            <p-skeleton width="4rem" height="1.75rem" />
                            <p-skeleton width="4rem" height="1.75rem" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonCardDemo {}
```

## Color

Customize the background color of the skeleton.

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="flex items-center justify-center">
            <div class="max-w-sm w-full space-y-4">
                <p-skeleton borderRadius="4px" class="bg-blue-500! dark:bg-blue-700!" />
                <p-skeleton borderRadius="4px" class="bg-red-500! dark:bg-red-700!" />
                <p-skeleton
                    borderRadius="4px"
                    class="bg-blue-100! dark:bg-blue-950! after:bg-[linear-gradient(90deg,rgba(255,255,255,0.01)_0%,oklch(0.82_0.18_260)_50%,rgba(255,255,255,0.01)_100%)] dark:after:bg-[linear-gradient(90deg,rgba(255,255,255,0)_0%,oklch(0.42_0.18_260)_50%,rgba(255,255,255,0)_100%)]"
                />
                <p-skeleton
                    borderRadius="4px"
                    class="bg-red-100! dark:bg-red-950! after:bg-[linear-gradient(90deg,rgba(255,255,255,0.01)_0%,oklch(63.7%_0.237_25.331)_50%,rgba(255,255,255,0.01)_100%)] dark:after:bg-[linear-gradient(90deg,rgba(255,255,255,0)_0%,oklch(63.7%_0.237_25.331)_50%,rgba(255,255,255,0)_100%)]"
                />
            </div>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonColorDemo {}
```

## DataTable

Sample DataTable implementation using different Skeleton components and Tailwind CSS utilities.

```typescript
import { Component, OnInit } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';
import { TableModule } from 'primeng/table';

@Component({
    template: `
        <p-table [value]="products" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Code</th>
                    <th>Name</th>
                    <th>Category</th>
                    <th>Quantity</th>
                </tr>
            </ng-template>
            <ng-template #body let-product>
                <tr>
                    <td><p-skeleton /></td>
                    <td><p-skeleton /></td>
                    <td><p-skeleton /></td>
                    <td><p-skeleton /></td>
                </tr>
            </ng-template>
        </p-table>
    `,
    standalone: true,
    imports: [SkeletonModule, TableModule]
})
export class SkeletonDataTableDemo implements OnInit {
    products: any[] | undefined;

    ngOnInit() {
        this.products = Array.from({ length: 5 }).map((_, i) => `Item #${i}`);
    }
}
```

## Grid

Sample Grid implementation using different Skeleton components and Tailwind CSS utilities.

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-8">
            @for (item of items; track $index) {
                <div class="p-4 rounded-lg bg-surface-50 dark:bg-surface-800/75">
                    <p-skeleton width="100%" height="10rem" />
                    <div class="mt-4 flex items-start gap-3">
                        <p-skeleton shape="circle" size="2.5rem" />
                        <div class="flex-1 flex flex-col gap-2">
                            <p-skeleton width="100%" borderRadius="4px" />
                            <p-skeleton width="90%" borderRadius="4px" />
                        </div>
                    </div>
                </div>
            }
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonGridDemo {
    items = Array.from({ length: 6 });
}
```

## List

Sample List implementation using different Skeleton components and Tailwind CSS utilities.

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="rounded-sm border border-surface-200 dark:border-surface-700 p-6 bg-surface-0 dark:bg-surface-900">
            <ul class="m-0 p-0 list-none">
                <li class="mb-4">
                    <div class="flex">
                        <p-skeleton shape="circle" size="4rem" class="mr-2" />
                        <div class="self-center" style="flex: 1">
                            <p-skeleton width="100%" class="mb-2" />
                            <p-skeleton width="75%" />
                        </div>
                    </div>
                </li>
                <li class="mb-4">
                    <div class="flex">
                        <p-skeleton shape="circle" size="4rem" class="mr-2" />
                        <div class="self-center" style="flex: 1">
                            <p-skeleton width="100%" class="mb-2" />
                            <p-skeleton width="75%" />
                        </div>
                    </div>
                </li>
                <li class="mb-4">
                    <div class="flex">
                        <p-skeleton shape="circle" size="4rem" class="mr-2" />
                        <div class="self-center" style="flex: 1">
                            <p-skeleton width="100%" class="mb-2" />
                            <p-skeleton width="75%" />
                        </div>
                    </div>
                </li>
                <li>
                    <div class="flex">
                        <p-skeleton shape="circle" size="4rem" class="mr-2" />
                        <div class="self-center" style="flex: 1">
                            <p-skeleton width="100%" class="mb-2" />
                            <p-skeleton width="75%" />
                        </div>
                    </div>
                </li>
            </ul>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonListDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="max-w-xs mx-auto flex flex-col gap-3">
            <p-skeleton borderRadius="0.75rem" class="aspect-video h-auto! w-full" />
            <div class="flex gap-3">
                <p-skeleton shape="circle" size="2.25rem" />
                <div class="flex-1 flex flex-col gap-1.5 py-0.5">
                    <p-skeleton width="100%" borderRadius="4px" class="h-auto! flex-1" />
                    <p-skeleton width="60%" borderRadius="4px" class="h-auto! flex-1" />
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonPreviewDemo {}
```

## Shapes

Various shapes and sizes can be created using styling properties like shape , width , height , borderRadius and class .

```typescript
import { Component } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';

@Component({
    template: `
        <div class="flex flex-col items-start gap-8 max-w-sm">
            <div class="w-full">
                <h5>Circle</h5>
                <div class="flex items-end gap-4">
                    <p-skeleton shape="circle" size="5rem" />
                    <p-skeleton shape="circle" size="4rem" />
                    <p-skeleton shape="circle" size="3rem" />
                    <p-skeleton shape="circle" size="2rem" />
                </div>
            </div>
            <div class="w-full">
                <h5>Square</h5>
                <div class="flex items-end gap-4">
                    <p-skeleton size="5rem" />
                    <p-skeleton size="4rem" />
                    <p-skeleton size="3rem" />
                    <p-skeleton size="2rem" />
                </div>
            </div>
            <div class="w-full">
                <h5>Rectangle</h5>
                <div class="flex flex-col gap-2 w-full">
                    <p-skeleton />
                    <p-skeleton width="12rem" />
                    <p-skeleton width="7rem" />
                    <p-skeleton height="4rem" />
                    <p-skeleton width="12rem" height="4rem" />
                </div>
            </div>
            <div class="w-full">
                <h5>Rounded</h5>
                <div class="flex flex-col gap-2 w-full">
                    <p-skeleton borderRadius="16px" />
                    <p-skeleton width="12rem" borderRadius="16px" />
                    <p-skeleton width="7rem" borderRadius="16px" />
                    <p-skeleton height="4rem" borderRadius="16px" />
                    <p-skeleton width="12rem" height="4rem" borderRadius="16px" />
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule]
})
export class SkeletonShapesDemo {}
```

## Skeleton

Skeleton is a placeholder to display instead of the actual content.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SkeletonPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| shape | string | - | Shape of the element. |
| animation | string | - | Type of the animation. |
| borderRadius | string \| undefined | - | Border radius of the element, defaults to value from theme. |
| size | string \| undefined | - | Size of the skeleton. |
| width | string | - | Width of the element. |
| height | string | - | Height of the element. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-skeleton | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| skeleton.border.radius | --p-skeleton-border-radius | Border radius of root |
| skeleton.background | --p-skeleton-background | Background of root |
| skeleton.animation.background | --p-skeleton-animation-background | Animation background of root |
