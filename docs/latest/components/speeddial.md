---
title: "Angular Speed Dial Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/speeddial.md"
description: "SpeedDial is a floating button with a popup menu."
---

# Angular Speed Dial Component

SpeedDial is a floating button with a popup menu.

## Accessibility

Screen Reader SpeedDial component renders a native button element that implicitly includes any passed prop. Text to describe the button can be defined with the aria-labelledby or aria-label props. Addititonally the button includes includes aria-haspopup , aria-expanded for states along with aria-controls to define the relation between the popup and the button. The popup overlay uses menu role on the list and each action item has a menuitem role with an aria-label as the menuitem label. The id of the menu refers to the aria-controls of the button.

## Basic

SpeedDial displays related actions from a single trigger button.

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem, MessageService } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 240px;">
            <p-speeddial [model]="items" direction="up" [style]="{ position: 'absolute', right: 0, bottom: 0 }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule],
    providers: [MessageService]
})
export class SpeedDialBasicDemo implements OnInit {
    private messageService = inject(MessageService);

    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [
            {
                icon: 'pi pi-pencil',
                command: () => this.messageService.add({ severity: 'info', summary: 'Add', detail: 'Data Added' })
            },
            {
                icon: 'pi pi-refresh',
                command: () => this.messageService.add({ severity: 'success', summary: 'Update', detail: 'Data Updated' })
            },
            {
                icon: 'pi pi-trash',
                command: () => this.messageService.add({ severity: 'error', summary: 'Delete', detail: 'Data Deleted' })
            },
            {
                icon: 'pi pi-upload'
            },
            {
                icon: 'pi pi-external-link',
                target: '_blank',
                url: 'https://angular.dev'
            }
        ];
    }
}
```

## Circle

Items can be displayed around the button when type is set to circle . Additional radius property defines the radius of the circle.

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex items-center justify-center" style="position: relative; height: 500px">
            <p-speeddial [model]="items" [radius]="80" type="circle" [style]="{ position: 'absolute' }" [buttonProps]="{ severity: 'warn', rounded: true }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialCircleDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }, { icon: 'pi pi-cog' }, { icon: 'pi pi-user' }, { icon: 'pi pi-heart' }];
    }
}
```

## Linear

When type is set to linear (default), items are displayed in a line based on the direction property. The direction can be up , down , left , or right .

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 500px">
            <p-speeddial [model]="items" direction="up" [style]="{ position: 'absolute', left: 'calc(50% - 2rem)', bottom: 0 }" />
            <p-speeddial [model]="items" direction="down" [style]="{ position: 'absolute', left: 'calc(50% - 2rem)', top: 0 }" />
            <p-speeddial [model]="items" direction="left" [style]="{ position: 'absolute', top: 'calc(50% - 2rem)', right: 0 }" />
            <p-speeddial [model]="items" direction="right" [style]="{ position: 'absolute', top: 'calc(50% - 2rem)', left: 0 }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialLinearDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }];
    }
}
```

## Mask

Adding mask property displays a modal layer behind the popup items.

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 360px">
            <p-speeddial [model]="items" direction="up" mask [style]="{ position: 'absolute', right: '1rem', bottom: '1rem' }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialMaskDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }];
    }
}
```

## preview-doc

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div class="relative h-105">
            <p-speeddial [model]="items" direction="down" [(visible)]="visibleDials[0]" (onVisibleChange)="onVisibleChange(0, $event)" [style]="{ position: 'absolute', top: 0, left: 0 }" />
            <p-speeddial [model]="items" direction="down" [(visible)]="visibleDials[1]" (onVisibleChange)="onVisibleChange(1, $event)" [style]="{ position: 'absolute', top: 0, right: 0 }" />
            <p-speeddial [model]="items" direction="up" [(visible)]="visibleDials[2]" (onVisibleChange)="onVisibleChange(2, $event)" [style]="{ position: 'absolute', bottom: 0, left: 0 }" />
            <p-speeddial [model]="items" direction="up" [(visible)]="visibleDials[3]" (onVisibleChange)="onVisibleChange(3, $event)" [style]="{ position: 'absolute', bottom: 0, right: 0 }" />
            <p-speeddial [model]="items" type="circle" [radius]="80" [(visible)]="visibleDials[4]" (onVisibleChange)="onVisibleChange(4, $event)" [style]="{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)' }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialPreviewDemo implements OnInit {
    items: MenuItem[] | undefined;

    visibleDials = [false, false, false, false, false];

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }];
    }

    onVisibleChange(index: number, visible: boolean) {
        if (visible) {
            this.visibleDials = this.visibleDials.map((_, i) => i === index);
        }
    }
}
```

## Quarter Circle

Setting type as quarter-circle displays the items at one of four corners of the button based on the direction .

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 500px">
            <p-speeddial [model]="items" [radius]="120" type="quarter-circle" direction="up-left" [style]="{ position: 'absolute', right: 0, bottom: 0 }" />
            <p-speeddial [model]="items" [radius]="120" type="quarter-circle" direction="up-right" [style]="{ position: 'absolute', left: 0, bottom: 0 }" />
            <p-speeddial [model]="items" [radius]="120" type="quarter-circle" direction="down-left" [style]="{ position: 'absolute', right: 0, top: 0 }" />
            <p-speeddial [model]="items" [radius]="120" type="quarter-circle" direction="down-right" [style]="{ position: 'absolute', left: 0, top: 0 }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialQuarterCircleDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }];
    }
}
```

## Semi Circle

When type is defined as semi-circle , items are displayed in a half-circle around the button.

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 500px">
            <p-speeddial [model]="items" [radius]="80" type="semi-circle" direction="up" [style]="{ position: 'absolute', left: 'calc(50% - 2rem)', bottom: 0 }" [buttonProps]="{ severity: 'success', rounded: true }" />
            <p-speeddial [model]="items" [radius]="80" type="semi-circle" direction="down" [style]="{ position: 'absolute', left: 'calc(50% - 2rem)', top: 0 }" [buttonProps]="{ severity: 'success', rounded: true }" />
            <p-speeddial [model]="items" [radius]="80" type="semi-circle" direction="left" [style]="{ position: 'absolute', top: 'calc(50% - 2rem)', right: 0 }" [buttonProps]="{ severity: 'success', rounded: true }" />
            <p-speeddial [model]="items" [radius]="80" type="semi-circle" direction="right" [style]="{ position: 'absolute', top: 'calc(50% - 2rem)', left: 0 }" [buttonProps]="{ severity: 'success', rounded: true }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialSemiCircleDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [{ icon: 'pi pi-pencil' }, { icon: 'pi pi-refresh' }, { icon: 'pi pi-trash' }, { icon: 'pi pi-upload' }, { icon: 'pi pi-external-link' }];
    }
}
```

## Template

SpeedDial supports full customization of the trigger button and action items using the button and item templates.

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center" style="position: relative; height: 450px">
            <p-speeddial [model]="items" direction="down" [transitionDelay]="50" [style]="{ position: 'absolute', top: 0, left: '50%', transform: 'translateX(-50%)' }" [buttonProps]="{ severity: 'warn', rounded: true }">
                <ng-template #item let-item let-toggleCallback="toggleCallback">
                    <div class="flex gap-3 cursor-pointer rounded-lg" (click)="toggleCallback($event, item)">
                        <span class="flex items-center justify-center w-24 px-6 py-2 bg-surface-0 dark:bg-surface-900 border border-surface-200 dark:border-surface-700 rounded-lg text-surface-500 dark:text-surface-400 font-medium">
                            {{ item.label }}
                        </span>
                        <span class="flex items-center justify-center w-12 h-12 rounded-full bg-surface-0 dark:bg-surface-900 border border-surface-200 dark:border-surface-700 text-surface-500 dark:text-surface-400">
                            <i [class]="item.icon + ' text-xl'"></i>
                        </span>
                    </div>
                </ng-template>
            </p-speeddial>
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialTemplateDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [
            {
                label: 'Like',
                icon: 'pi pi-heart'
            },
            {
                label: 'Share',
                icon: 'pi pi-share-alt'
            },
            {
                label: 'Print',
                icon: 'pi pi-print'
            },
            {
                label: 'Save',
                icon: 'pi pi-save'
            },
            {
                label: 'Copy',
                icon: 'pi pi-copy'
            }
        ];
    }
}
```

## Tooltip

SpeedDial can be combined with Tooltip to display labels for action items.

```typescript
import { Component, OnInit } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem } from 'primeng/api';

@Component({
    template: `
        <div style="position: relative; height: 350px">
            <p-speeddial [model]="items" direction="up" [style]="{ position: 'absolute', right: 0, bottom: 0 }" [buttonProps]="{ severity: 'help', rounded: true }" [tooltipOptions]="{ tooltipPosition: 'left' }" />
            <p-speeddial [model]="items" direction="up" [style]="{ position: 'absolute', left: 0, bottom: 0 }" [buttonProps]="{ severity: 'danger', rounded: true }" [tooltipOptions]="{ tooltipPosition: 'right' }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule]
})
export class SpeedDialTooltipDemo implements OnInit {
    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [
            {
                label: 'Add',
                icon: 'pi pi-pencil'
            },
            {
                label: 'Update',
                icon: 'pi pi-refresh'
            },
            {
                label: 'Delete',
                icon: 'pi pi-trash'
            },
            {
                label: 'Upload',
                icon: 'pi pi-upload'
            },
            {
                label: 'External',
                icon: 'pi pi-external-link'
            }
        ];
    }
}
```

## Transition Delay

The transitionDelay property specifies the delay in milliseconds between each action item's appearance animation.

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { SpeedDialModule } from 'primeng/speeddial';
import { MenuItem, MessageService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-between" style="position: relative; height: 240px;">
            <p-speeddial [model]="items" direction="up" [transitionDelay]="0" [style]="{ position: 'absolute', left: 0, bottom: 0 }" [buttonProps]="{ severity: 'secondary', rounded: true }" />
            <p-speeddial [model]="items" direction="up" [transitionDelay]="80" [style]="{ position: 'absolute', left: 'calc(50% - 2rem)', bottom: 0 }" [buttonProps]="{ severity: 'success', rounded: true }" />
            <p-speeddial [model]="items" direction="up" [transitionDelay]="150" [style]="{ position: 'absolute', right: 0, bottom: 0 }" [buttonProps]="{ severity: 'info', rounded: true }" />
        </div>
    `,
    standalone: true,
    imports: [SpeedDialModule],
    providers: [MessageService]
})
export class SpeedDialTransitionDelayDemo implements OnInit {
    private messageService = inject(MessageService);

    items: MenuItem[] | undefined;

    ngOnInit() {
        this.items = [
            {
                icon: 'pi pi-pencil',
                command: () => this.messageService.add({ severity: 'info', summary: 'Add', detail: 'Data Added' })
            },
            {
                icon: 'pi pi-refresh',
                command: () => this.messageService.add({ severity: 'success', summary: 'Update', detail: 'Data Updated' })
            },
            {
                icon: 'pi pi-trash',
                command: () => this.messageService.add({ severity: 'error', summary: 'Delete', detail: 'Data Deleted' })
            },
            {
                icon: 'pi pi-upload'
            },
            {
                icon: 'pi pi-external-link',
                target: '_blank',
                url: 'https://angular.dev'
            }
        ];
    }
}
```

## Speed Dial

When pressed, a floating action button can display multiple primary actions that can be performed on a page.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SpeedDialPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| id | string \| undefined | - | List of items id. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the element. |
| className | string \| undefined | - | Style class of the element. |
| $model | MenuItem[] \| null | - | MenuModel instance to define the action items. |
| visible | boolean | false | Specifies the visibility of the overlay. |
| transitionDelay | number | - | Transition delay step for each action item. |
| type | "linear" \| "circle" \| "semi-circle" \| "quarter-circle" | - | Specifies the opening type of actions. |
| radius | number | - | Radius for *circle types. |
| mask | boolean | - | Whether to show a mask element behind the speeddial. |
| disabled | boolean | - | Whether the component is disabled. |
| hideOnClickOutside | boolean | - | Whether the actions close when clicked outside. |
| buttonStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the button element. |
| buttonClassName | string \| undefined | - | Style class of the button element. |
| maskStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the mask element. |
| maskClassName | string \| undefined | - | Style class of the mask element. |
| showIcon | string \| undefined | - | Show icon of the button element. |
| hideIcon | string \| undefined | - | Hide icon of the button element. |
| rotateAnimation | boolean | - | Defined to rotate showIcon when hideIcon is not present. |
| ariaLabel | string \| undefined | - | Defines a string value that labels an interactive element. |
| ariaLabelledBy | string \| undefined | - | Identifier of the underlying input element. |
| tooltipOptions | TooltipOptions \| undefined | - | Whether to display the tooltip on items. The modifiers of Tooltip can be used like an object in it. Valid keys are 'event' and 'position'. |
| buttonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the Button component. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onVisibleChange | value: boolean | Fired when the visibility of element changed. |
| onClick | event: MouseEvent | Fired when the button element clicked. |
| onShow | value: void | Fired when the actions are visible. |
| onHide | value: void | Fired when the actions are hidden. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| button | TemplateRef<SpeedDialButtonTemplateContext> \| undefined | Custom button template. |
| item | TemplateRef<SpeedDialItemTemplateContext> \| undefined | Custom item template. |
| icon | TemplateRef<void> \| undefined | Custom icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| pcButton | ButtonPassThrough | Used to pass attributes to the Button component. |
| list | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the list's DOM element. |
| item | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the item's DOM element. |
| pcAction | ButtonPassThrough | Used to pass attributes to the action's Button component. |
| actionIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the action icon's DOM element. |
| mask | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the mask's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-speeddial | Class name of the root element |
| p-speeddial-button | Class name of the button element |
| p-speeddial-list | Class name of the list element |
| p-speeddial-item | Class name of the item element |
| p-speeddial-action | Class name of the action element |
| p-speeddial-action-icon | Class name of the action icon element |
| p-speeddial-mask | Class name of the mask element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| speeddial.gap | --p-speeddial-gap | Gap of root |
| speeddial.transition.duration | --p-speeddial-transition-duration | Transition duration of root |
