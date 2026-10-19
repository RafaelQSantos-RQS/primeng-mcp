---
title: "Angular MeterGroup Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/metergroup.md"
description: "MeterGroup displays scalar measurements within a known range."
---

# Angular MeterGroup Component

MeterGroup displays scalar measurements within a known range.

## Accessibility

Screen Reader MeterGroup component uses meter role in addition to the aria-valuemin , aria-valuemax and aria-valuenow attributes. Value to describe the component can be defined using aria-labelledby prop. Keyboard Support Component does not include any interactive elements.

## Basic

Visualizes multiple values as segmented horizontal bars.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupBasicDemo {
    value = [{ label: 'Space used', value: 15, color: 'var(--p-primary-color)' }];
}
```

## Color

MeterGroup supports custom color values. Use color property with hex, rgb, hsl, or hsla values.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupColorDemo {
    value = [
        { label: 'Violet', value: 12, color: 'var(--p-violet-500)' },
        { label: 'Emerald', value: 14, color: '#10B981' },
        { label: 'Rose', value: 10, color: 'rgb(244, 63, 94)' },
        { label: 'Blue', value: 8, color: '#3B82F6' },
        { label: 'Yellow', value: 10, color: '#EAB308' }
    ];
}
```

## Icon

Icons can be displayed next to the labels instead of the default marker.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupIconDemo {
    value = [
        { label: 'Apps', color: 'var(--p-violet-500)', value: 16, icon: 'pi pi-table' },
        { label: 'Messages', color: 'var(--p-emerald-500)', value: 8, icon: 'pi pi-inbox' },
        { label: 'Media', color: 'var(--p-amber-500)', value: 24, icon: 'pi pi-image' },
        { label: 'System', color: 'var(--p-blue-500)', value: 10, icon: 'pi pi-cog' }
    ];
}
```

## Label

The default orientation of the labels is horizontal, and the vertical alternative is available through the labelOrientation option.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" labelPosition="start" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupLabelDemo {
    value = [
        { label: 'Apps', color: 'var(--p-violet-500)', value: 16 },
        { label: 'Messages', color: 'var(--p-emerald-500)', value: 8 },
        { label: 'Media', color: 'var(--p-amber-500)', value: 24 },
        { label: 'System', color: 'var(--p-blue-500)', value: 10 }
    ];
}
```

## Min Max

Boundaries are configured with the min and max values whose defaults are 0 and 100 respectively.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" [max]="200" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupMinMaxDemo {
    value = [
        { label: 'Apps', color: 'var(--p-violet-500)', value: 16 },
        { label: 'Messages', color: 'var(--p-emerald-500)', value: 8 },
        { label: 'Media', color: 'var(--p-amber-500)', value: 24 },
        { label: 'System', color: 'var(--p-blue-500)', value: 10 }
    ];
}
```

## Multiple

Adding more items to the value array displays the meters in a group.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <p-metergroup [value]="value" class="max-w-md mx-auto" />
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupMultipleDemo {
    value = [
        { label: 'Apps', color: 'var(--p-violet-500)', value: 14 },
        { label: 'Messages', color: 'var(--p-emerald-500)', value: 12 },
        { label: 'Media', color: 'var(--p-amber-500)', value: 8 },
        { label: 'System', color: 'var(--p-blue-500)', value: 12 },
        { label: 'Documents', color: 'var(--p-gray-500)', value: 6 },
        { label: 'Cache', color: 'var(--p-cyan-500)', value: 11 },
        { label: 'Other', color: 'var(--p-pink-500)', value: 9 }
    ];
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <div class="max-w-md mx-auto">
            <div class="mb-4 font-mono text-sm text-color font-medium uppercase tracking-wider">Disk usage</div>
            <p-metergroup [value]="value" />
        </div>
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupPreviewDemo {
    value = [
        { label: 'Apps', value: 14, color: 'var(--p-violet-500)' },
        { label: 'Messages', value: 12, color: 'var(--p-emerald-500)' },
        { label: 'Media', value: 8, color: 'var(--p-amber-500)' },
        { label: 'System', value: 12, color: 'var(--p-blue-500)' },
        { label: 'Documents', value: 6, color: 'var(--p-gray-500)' },
        { label: 'Cache', value: 11, color: 'var(--p-cyan-500)' },
        { label: 'Other', value: 9, color: 'var(--p-pink-500)' }
    ];
}
```

## Template

MeterGroup provides templating support for labels, meter items, and content around the meters.

```typescript
import { Component } from '@angular/core';
import { CardModule } from 'primeng/card';
import { MeterGroupModule } from 'primeng/metergroup';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <p-metergroup [value]="value" labelPosition="start" class="max-w-sm mx-auto">
            <ng-template #label>
                <div class="flex flex-wrap gap-4">
                    @for (meterItem of value; track $index; let index = $index) {
                        <p-card class="flex-1 border border-surface shadow-none">
                            <div class="flex justify-between gap-8">
                                <div class="flex flex-col gap-1">
                                    <span class="text-surface-500 dark:text-surface-400 text-sm">{{ meterItem.label }}</span>
                                    <span class="font-bold text-lg">{{ meterItem.value }}%</span>
                                </div>
                                <span class="w-8 h-8 rounded-full inline-flex justify-center items-center text-center" [style]="{ 'background-color': meterItem.color1, color: '#ffffff' }">
                                    <i [class]="meterItem.icon"></i>
                                </span>
                            </div>
                        </p-card>
                    }
                </div>
            </ng-template>
            <ng-template #meter let-value let-class="class" let-width="size">
                <span [class]="class" [style]="{ background: 'linear-gradient(to right, ' + value.color1 + ', ' + value.color2 + ')', width: width }"></span>
            </ng-template>
            <ng-template #start let-totalPercent="totalPercent">
                <div class="flex justify-between mt-4 mb-2 relative">
                    <span class="text-sm">Storage</span>
                    <span [style]="{ width: totalPercent + '%' }" class="absolute text-right text-sm">{{ totalPercent }}%</span>
                    <span class="font-medium text-sm">1TB</span>
                </div>
            </ng-template>
            <ng-template #end>
                <div class="flex justify-between mt-4">
                    <button type="button" pButton [outlined]="true" size="small">Manage Storage</button>
                    <button type="button" pButton size="small">Update Plan</button>
                </div>
            </ng-template>
        </p-metergroup>
    `,
    standalone: true,
    imports: [CardModule, MeterGroupModule, ButtonModule]
})
export class MeterGroupTemplateDemo {
    value = [
        { label: 'Apps', color1: '#34d399', color2: '#fbbf24', value: 25, icon: 'pi pi-table' },
        { label: 'Messages', color1: '#fbbf24', color2: '#60a5fa', value: 15, icon: 'pi pi-inbox' },
        { label: 'Media', color1: '#60a5fa', color2: '#c084fc', value: 20, icon: 'pi pi-image' },
        { label: 'System', color1: '#c084fc', color2: '#c084fc', value: 10, icon: 'pi pi-cog' }
    ];
}
```

## Vertical

Layout of the MeterGroup is configured with the orientation property that accepts either horizontal or vertical as available options.

```typescript
import { Component } from '@angular/core';
import { MeterGroupModule } from 'primeng/metergroup';

@Component({
    template: `
        <div class="flex justify-center">
            <p-metergroup [value]="value" orientation="vertical" labelOrientation="vertical" [style]="{ height: '360px' }" />
        </div>
    `,
    standalone: true,
    imports: [MeterGroupModule]
})
export class MeterGroupVerticalDemo {
    value = [
        { label: 'Apps', color: 'var(--p-violet-500)', value: 24 },
        { label: 'Messages', color: 'var(--p-emerald-500)', value: 16 },
        { label: 'Media', color: 'var(--p-amber-500)', value: 24 },
        { label: 'System', color: 'var(--p-blue-500)', value: 12 }
    ];
}
```

## Meter Group

MeterGroup displays scalar measurements within a known range.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, MeterGroupPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | MeterItem[] \| undefined | - | Current value of the metergroup. |
| min | number | - | Mininum boundary value. |
| max | number | - | Maximum boundary value. |
| orientation | "horizontal" \| "vertical" | - | Specifies the layout of the component, valid values are 'horizontal' and 'vertical'. |
| labelPosition | "start" \| "end" | - | Specifies the label position of the component, valid values are 'start' and 'end'. |
| labelOrientation | "horizontal" \| "vertical" | - | Specifies the label orientation of the component, valid values are 'horizontal' and 'vertical'. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| label | TemplateRef<MeterGroupLabelTemplateContext> \| undefined | Custom label template. |
| meter | TemplateRef<MeterGroupMeterTemplateContext> \| undefined | Custom meter template. |
| end | TemplateRef<MeterGroupLabelTemplateContext> \| undefined | Custom end template. |
| start | TemplateRef<MeterGroupLabelTemplateContext> \| undefined | Custom start template. |
| icon | TemplateRef<MeterGroupIconTemplateContext> \| undefined | Custom icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| meters | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the meters' DOM element. |
| meter | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the meter's DOM element. |
| labelList | PassThroughOption<HTMLOListElement, I> | Used to pass attributes to the label list's DOM element. |
| label | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the label's DOM element. |
| labelIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the label icon's DOM element. |
| labelMarker | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the label marker's DOM element. |
| labelText | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the label text's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-metergroup | Class name of the root element |
| p-metergroup-meters | Class name of the meters element |
| p-metergroup-meter | Class name of the meter element |
| p-metergroup-label-list | Class name of the label list element |
| p-metergroup-label | Class name of the label element |
| p-metergroup-label-icon | Class name of the label icon element |
| p-metergroup-label-marker | Class name of the label marker element |
| p-metergroup-label-text | Class name of the label text element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| metergroup.border.radius | --p-metergroup-border-radius | Border radius of root |
| metergroup.gap | --p-metergroup-gap | Gap of root |
| metergroup.meters.background | --p-metergroup-meters-background | Background of meters |
| metergroup.meters.size | --p-metergroup-meters-size | Size of meters |
| metergroup.label.gap | --p-metergroup-label-gap | Gap of label |
| metergroup.label.marker.size | --p-metergroup-label-marker-size | Size of label marker |
| metergroup.label.text.font.weight | --p-metergroup-label-text-font-weight | Font weight of label text |
| metergroup.label.text.font.size | --p-metergroup-label-text-font-size | Font size of label text |
| metergroup.label.icon.size | --p-metergroup-label-icon-size | Size of label icon |
| metergroup.label.list.vertical.gap | --p-metergroup-label-list-vertical-gap | Vertical gap of label list |
| metergroup.label.list.horizontal.gap | --p-metergroup-label-list-horizontal-gap | Horizontal gap of label list |
