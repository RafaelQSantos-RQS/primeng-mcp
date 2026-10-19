---
title: "Angular Breadcrumb Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/breadcrumb.md"
description: "Breadcrumb provides contextual information about page hierarchy."
---

# Angular Breadcrumb Component

Breadcrumb provides contextual information about page hierarchy.

## Accessibility

Screen Reader Breadcrumb uses the nav element and since any attribute is passed to the root implicitly aria-labelledby or aria-label can be used to describe the component. Inside an ordered list is used where the list item separators have aria-hidden to be able to ignored by the screen readers. If the last link represents the current route, aria-current is added with "page" as the value. Keyboard Support No special keyboard interaction is needed, all menuitems are focusable based on the page tab sequence.

## Basic

Shows the current location within a navigational hierarchy.

```typescript
import { Component } from '@angular/core';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [model]="items" [home]="home" />
        </div>
    `,
    standalone: true,
    imports: [BreadcrumbModule]
})
export class BreadcrumbBasicDemo {
    items: MenuItem[] = [{ label: 'Products' }, { icon: 'pi pi-bolt', label: 'Electronics' }, { label: 'Laptops' }, { label: 'Dell' }];

    home: MenuItem = { icon: 'pi pi-home' };
}
```

## Custom Separator

A breadcrumb allows customization of the separator between items using the separator template.

```typescript
import { Component } from '@angular/core';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [model]="items" [home]="home">
                <ng-template #separator> / </ng-template>
            </p-breadcrumb>
        </div>
    `,
    standalone: true,
    imports: [BreadcrumbModule]
})
export class BreadcrumbCustomSeparatorDemo {
    items: MenuItem[] = [{ label: 'Products' }, { icon: 'pi pi-bolt', label: 'Electronics' }, { label: 'Laptops' }, { label: 'Dell' }];

    home: MenuItem = { icon: 'pi pi-home' };
}
```

## Ellipsis

An ellipsis can be used to indicate hidden breadcrumb items.

```typescript
import { Component } from '@angular/core';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [model]="items" [home]="home">
                <ng-template #item let-item>
                    @if (item.current) {
                        <span class="p-breadcrumb-item-link font-semibold">{{ item.label }}</span>
                    } @else if (item.icon && !item.label) {
                        <a class="p-breadcrumb-item-link cursor-pointer"><svg [pIcon]="item.icon"></svg></a>
                    } @else {
                        <a class="p-breadcrumb-item-link cursor-pointer flex items-center gap-1">
                            @if (item.icon) {
                                <svg [pIcon]="item.icon"></svg>
                            }
                            @if (item.label) {
                                <span>{{ item.label }}</span>
                            }
                        </a>
                    }
                </ng-template>
            </p-breadcrumb>
        </div>
    `,
    standalone: true,
    imports: [BreadcrumbModule, PIcon]
})
export class BreadcrumbEllipsisDemo {
    items: MenuItem[] = [{ icon: 'ellipsis-h' }, { icon: 'bolt', label: 'Electronics' }, { label: 'Laptops' }, { label: 'Dell', current: true }];

    home: MenuItem = { icon: 'home' };
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [model]="items" [home]="home">
                <ng-template #item let-item>
                    @if (item.icon && !item.label) {
                        <a class="p-breadcrumb-item-link cursor-pointer"><svg [pIcon]="item.icon"></svg></a>
                    } @else {
                        <a class="p-breadcrumb-item-link cursor-pointer flex items-center gap-1">
                            @if (item.icon) {
                                <svg [pIcon]="item.icon"></svg>
                            }
                            @if (item.label) {
                                <span>{{ item.label }}</span>
                            }
                        </a>
                    }
                </ng-template>
            </p-breadcrumb>
        </div>
    `,
    standalone: true,
    imports: [BreadcrumbModule, PIcon]
})
export class BreadcrumbPreviewDemo {
    items: MenuItem[] = [{ icon: 'ellipsis-h' }, { label: 'Products' }, { icon: 'bolt', label: 'Electronics' }, { label: 'Laptops' }, { label: 'Dell' }];

    home: MenuItem = { icon: 'home' };
}
```

## Route

A breadcrumb can be used with routing libraries to navigate between pages.

```typescript
import { Component, OnInit } from '@angular/core';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [home]="home" [model]="items">
                <ng-template #item let-item>
                    @if (item.current) {
                        <span class="p-breadcrumb-item-link font-semibold">{{ item.label }}</span>
                    } @else if (item.icon && !item.label) {
                        <a class="p-breadcrumb-item-link cursor-pointer" (click)="navigate(item)"><svg [pIcon]="item.icon"></svg></a>
                    } @else {
                        <a class="p-breadcrumb-item-link cursor-pointer" (click)="navigate(item)">{{ item.label }}</a>
                    }
                </ng-template>
            </p-breadcrumb>
        </div>
    `,
    standalone: true,
    imports: [BreadcrumbModule, PIcon]
})
export class BreadcrumbRouterDemo implements OnInit {
    allPages: any[] = [{ label: 'Home', icon: 'home' }, { label: 'Components' }, { label: 'Form' }, { label: 'Input' }, { label: 'InputText' }, { label: 'Variants' }, { label: 'Filled' }, { label: 'Outlined' }];

    home!: MenuItem;

    items!: MenuItem[];

    ngOnInit() {
        this.updateBreadcrumb(this.allPages);
    }

    navigate(item: any) {
        const index = item.icon && !item.label ? 0 : this.allPages.findIndex((p) => p.label === item.label);

        if (index !== -1) {
            this.updateBreadcrumb(this.allPages.slice(0, index + 1));
        }
    }

    updateBreadcrumb(pages: any[]) {
        if (pages.length === 1) {
            this.home = null as any;
            this.items = [{ label: pages[0].label, current: true }] as any[];
        } else {
            this.home = { icon: pages[0].icon } as any;
            this.items = pages.slice(1).map((page, index, arr) => ({
                ...page,
                current: index === arr.length - 1
            })) as any[];
        }
    }
}
```

## Custom Item

Custom content can be placed inside the items using the item template.

```typescript
import { Component } from '@angular/core';
import { BadgeModule } from 'primeng/badge';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <div class="flex justify-center">
            <p-breadcrumb [model]="items" [home]="home">
                <ng-template #item let-item>
                    <a class="p-breadcrumb-item-link flex items-center gap-1 cursor-pointer">
                        @if (item.icon) {
                            <svg [pIcon]="item.icon"></svg>
                        }
                        @if (item.label) {
                            <span>{{ item.label }}</span>
                        }
                        @if (item.badge) {
                            <p-badge [value]="item.badge" shape="circle" />
                        }
                    </a>
                </ng-template>
            </p-breadcrumb>
        </div>
    `,
    standalone: true,
    imports: [BadgeModule, BreadcrumbModule, PIcon]
})
export class BreadcrumbTemplateDemo {
    items: MenuItem[] = [{ label: 'Products' }, { icon: 'bolt', label: 'Electronics' }, { icon: 'desktop', label: 'Computers' }, { label: 'Laptops', badge: '5' }, { label: 'Dell' }];

    home: MenuItem = { icon: 'home', label: 'Home' };
}
```

## Breadcrumb

Breadcrumb provides contextual information about page hierarchy.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, BreadcrumbPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| model | MenuItem[] \| undefined | - | An array of menuitems. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| styleClass | string \| undefined | - | Style class of the component. |
| home | MenuItem \| undefined | - | MenuItem configuration for the home icon. |
| homeAriaLabel | string \| undefined | - | Defines a string that labels the home icon for accessibility. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onItemClick | event: BreadcrumbItemClickEvent | Fired when an item is selected. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<BreadcrumbItemTemplateContext> \| undefined | Custom item template. |
| separator | TemplateRef<void> \| undefined | Custom separator template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| list | PassThroughOption<HTMLOListElement, I> | Used to pass attributes to the list's DOM element. |
| homeItem | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the home item's DOM element. |
| item | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the item's DOM element. |
| itemLink | PassThroughOption<HTMLAnchorElement, I> | Used to pass attributes to the item link's DOM element. |
| itemIcon | PassThroughOption<HTMLSpanElement \| SVGElement, I> | Used to pass attributes to the item icon's DOM element. |
| itemLabel | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the item label's DOM element. |
| separator | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the separator's DOM element. |
| separatorIcon | PassThroughOption<SVGElement, I> | Used to pass attributes to the separator icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-breadcrumb | Class name of the root element |
| p-breadcrumb-list | Class name of the list element |
| p-breadcrumb-home-item | Class name of the home item element |
| p-breadcrumb-separator | Class name of the separator element |
| p-breadcrumb-item | Class name of the item element |
| p-breadcrumb-item-link | Class name of the item link element |
| p-breadcrumb-item-icon | Class name of the item icon element |
| p-breadcrumb-item-label | Class name of the item label element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| breadcrumb.padding | --p-breadcrumb-padding | Padding of root |
| breadcrumb.background | --p-breadcrumb-background | Background of root |
| breadcrumb.gap | --p-breadcrumb-gap | Gap of root |
| breadcrumb.transition.duration | --p-breadcrumb-transition-duration | Transition duration of root |
| breadcrumb.item.color | --p-breadcrumb-item-color | Color of item |
| breadcrumb.item.hover.color | --p-breadcrumb-item-hover-color | Hover color of item |
| breadcrumb.item.border.radius | --p-breadcrumb-item-border-radius | Border radius of item |
| breadcrumb.item.gap | --p-breadcrumb-item-gap | Gap of item |
| breadcrumb.item.icon.color | --p-breadcrumb-item-icon-color | Icon color of item |
| breadcrumb.item.icon.hover.color | --p-breadcrumb-item-icon-hover-color | Icon hover color of item |
| breadcrumb.item.icon.size | --p-breadcrumb-item-icon-size | Icon size of item icon |
| breadcrumb.item.label.font.weight | --p-breadcrumb-item-label-font-weight | Font weight of item label |
| breadcrumb.item.label.font.size | --p-breadcrumb-item-label-font-size | Font size of item label |
| breadcrumb.item.focus.ring.width | --p-breadcrumb-item-focus-ring-width | Focus ring width of item |
| breadcrumb.item.focus.ring.style | --p-breadcrumb-item-focus-ring-style | Focus ring style of item |
| breadcrumb.item.focus.ring.color | --p-breadcrumb-item-focus-ring-color | Focus ring color of item |
| breadcrumb.item.focus.ring.offset | --p-breadcrumb-item-focus-ring-offset | Focus ring offset of item |
| breadcrumb.item.focus.ring.shadow | --p-breadcrumb-item-focus-ring-shadow | Focus ring shadow of item |
| breadcrumb.separator.color | --p-breadcrumb-separator-color | Color of separator |
