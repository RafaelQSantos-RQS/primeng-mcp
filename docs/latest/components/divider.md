---
title: "Angular Divider Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/divider.md"
description: "Divider is used to separate contents."
---

# Angular Divider Component

Divider is used to separate contents.

## Accessibility

Screen Reader Divider uses a separator role with aria-orientation set to either "horizontal" or "vertical". Keyboard Support Component does not include any interactive elements.

## Basic

Divider is placed between items to separate content.

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="max-w-sm w-full text-sm">
                <div class="flex items-center justify-between">
                    <span class="text-color font-medium">Subtotal</span>
                    <span class="text-color">$89.00</span>
                </div>
                <p-divider />
                <div class="flex items-center justify-between">
                    <span class="text-color font-medium">Shipping</span>
                    <span class="text-color">$5.99</span>
                </div>
                <p-divider />
                <div class="flex items-center justify-between">
                    <span class="text-color font-semibold">Total</span>
                    <span class="text-color font-semibold">$94.99</span>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule]
})
export class DividerBasicDemo {}
```

## Content

Children are rendered within the boundaries of the divider where location of the content is configured with the align property. In horizontal layout, alignment options are left , center and right whereas vertical mode supports top , center and bottom .

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="max-w-md w-full">
                <p class="text-sm">Fast setup with a simple onboarding process, no credit card required to get started.</p>
                <p-divider align="left" type="solid">
                    <code class="uppercase text-xs">Getting started</code>
                </p-divider>
                <p class="text-sm">Cancel anytime directly from your account settings, with no questions asked.</p>
                <p-divider align="center" type="dotted">
                    <code class="uppercase text-xs">Flexibility</code>
                </p-divider>
                <p class="text-sm">24/7 support included to help you resolve issues quickly, whenever you need assistance.</p>
                <p-divider align="right" type="dashed">
                    <code class="uppercase text-xs">Support</code>
                </p-divider>
                <p class="text-sm">No long-term commitments or hidden contracts, just transparent and flexible pricing.</p>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule]
})
export class DividerContentDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="max-w-md w-full">
                <div class="flex">
                    <div>
                        <div class="uppercase font-mono text-xs">Invoice No</div>
                        <div class="font-light">0000123</div>
                    </div>
                    <p-divider layout="vertical" />
                    <div>
                        <div class="uppercase font-mono text-xs">Issued</div>
                        <div class="font-light">01/01/2026</div>
                    </div>
                    <p-divider layout="vertical" />
                    <div>
                        <div class="uppercase font-mono text-xs">Due Date</div>
                        <div class="font-light">02/02/2026</div>
                    </div>
                </div>
                <p-divider />
                <div class="flex">
                    <div class="flex-1">
                        <div class="uppercase font-mono text-xs">From</div>
                        <div class="font-semibold text-2xl">PrimeTek</div>
                        <div class="w-full h-12 mt-4 bg-surface-100 dark:bg-surface-900 rounded-lg"></div>
                    </div>
                    <p-divider layout="vertical" />
                    <div class="flex-1">
                        <div class="uppercase font-mono text-xs">To</div>
                        <div class="h-7 mt-2 rounded-lg bg-surface-100 dark:bg-surface-900"></div>
                        <div class="w-full h-12 mt-2 bg-surface-100 dark:bg-surface-900 rounded-lg"></div>
                    </div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule]
})
export class DividerPreviewDemo {}
```

## Type

Style of the border is configured with the type property that can be solid , dotted or dashed .

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="max-w-md w-full">
                <p class="text-sm">Fast setup, no credit card required</p>
                <p-divider type="solid" />
                <p class="text-sm">Cancel anytime from your account</p>
                <p-divider type="dotted" />
                <p class="text-sm">24/7 support included</p>
                <p-divider type="dashed" />
                <p class="text-sm">No long-term commitments</p>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule]
})
export class DividerTypeDemo {}
```

## Vertical

Vertical divider is enabled by setting the layout property as vertical .

```typescript
import { Component } from '@angular/core';
import { DividerModule } from 'primeng/divider';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex w-fit">
                <div>
                    <div class="uppercase font-mono text-xs">Invoice No</div>
                    <div class="font-light">0000123</div>
                </div>
                <p-divider layout="vertical" />
                <div>
                    <div class="uppercase font-mono text-xs">Issued</div>
                    <div class="font-light">01/01/2026</div>
                </div>
                <p-divider layout="vertical" />
                <div>
                    <div class="uppercase font-mono text-xs">Due Date</div>
                    <div class="font-light">02/02/2026</div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [DividerModule]
})
export class DividerVerticalDemo {}
```

## Divider

Divider is used to separate contents.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, DividerPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| layout | "horizontal" \| "vertical" | - | Specifies the orientation. |
| type | "solid" \| "dashed" \| "dotted" | - | Border style type. |
| align | DividerAlign \| undefined | - | Alignment of the content. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-divider | Class name of the root element |
| p-divider-content | Class name of the content element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| divider.border.color | --p-divider-border-color | Border color of root |
| divider.content.background | --p-divider-content-background | Background of content |
| divider.content.color | --p-divider-content-color | Color of content |
| divider.horizontal.margin | --p-divider-horizontal-margin | Margin of horizontal |
| divider.horizontal.padding | --p-divider-horizontal-padding | Padding of horizontal |
| divider.horizontal.content.padding | --p-divider-horizontal-content-padding | Content padding of horizontal |
| divider.vertical.margin | --p-divider-vertical-margin | Margin of vertical |
| divider.vertical.padding | --p-divider-vertical-padding | Padding of vertical |
| divider.vertical.content.padding | --p-divider-vertical-content-padding | Content padding of vertical |
