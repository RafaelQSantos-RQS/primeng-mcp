---
title: "Angular Organization Chart Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/organizationchart.md"
description: "OrganizationChart visualizes hierarchical organization data."
---

# Angular Organization Chart Component

OrganizationChart visualizes hierarchical organization data.

## Accessibility

Screen Reader Component uses ARIA roles and attributes for screen reader accessibility. The root element has role="tree" with aria-multiselectable for multiple selection support. Each tree item uses role="treeitem" with aria-level for hierarchy, aria-expanded for collapse state, and aria-selected for selection state. Child nodes are grouped with role="group" . Keyboard Support Node Key Function tab Moves focus through the focusable nodes within the chart. enter Toggles the selection state of a node. space Toggles the selection state of a node. Collapse Button Key Function tab Moves focus through the focusable elements within the chart. enter Toggles the expanded state of a node. space Toggles the expanded state of a node.

## Basic

Displays a hierarchy from the value collection.

```typescript
import { Component } from '@angular/core';
import { OrganizationChartModule } from 'primeng/organizationchart';
import { Product } from '@/domain/product';

@Component({
    template: `
        <div class="flex items-center justify-center overflow-x-auto">
            <p-organization-chart [value]="data" />
        </div>
    `,
    standalone: true,
    imports: [OrganizationChartModule]
})
export class OrganizationChartBasicDemo {
    data: OrgChartNode[] = [
        {
            key: '0',
            label: 'Founder',
            children: [
                {
                    key: '0-0',
                    label: 'Product Lead',
                    children: [
                        {
                            key: '0-0-0',
                            label: 'UX/UI Designer'
                        },
                        {
                            key: '0-0-1',
                            label: 'Product Manager'
                        }
                    ]
                },
                {
                    key: '0-1',
                    label: 'Engineering Lead',
                    children: [
                        {
                            key: '0-1-0',
                            label: 'Frontend Developer'
                        },
                        {
                            key: '0-1-1',
                            label: 'Backend Developer'
                        }
                    ]
                }
            ]
        }
    ];
}
```

## Collapsible

Use the collapsible property to make nodes collapsible.

```typescript
import { Component } from '@angular/core';
import { OrganizationChartModule } from 'primeng/organizationchart';
import { Product } from '@/domain/product';

@Component({
    template: `
        <div class="flex items-center justify-center overflow-x-auto">
            <p-organization-chart [value]="data" [collapsible]="true" />
        </div>
    `,
    standalone: true,
    imports: [OrganizationChartModule]
})
export class OrganizationChartCollapsibleDemo {
    data: OrgChartNode[] = [
        {
            key: '0',
            label: 'Founder',
            children: [
                {
                    key: '0-0',
                    label: 'Product Lead',
                    children: [
                        {
                            key: '0-0-0',
                            label: 'UX/UI Designer'
                        },
                        {
                            key: '0-0-1',
                            label: 'Product Manager'
                        }
                    ]
                },
                {
                    key: '0-1',
                    label: 'Engineering Lead',
                    children: [
                        {
                            key: '0-1-0',
                            label: 'Frontend Developer'
                        },
                        {
                            key: '0-1-1',
                            label: 'Backend Developer'
                        }
                    ]
                }
            ]
        }
    ];
}
```

## Content

The #node template customizes the node content. OrganizationChart still renders its own structure around it — the content is placed inside .p-organizationchart-node-content , and the built-in collapse button is preserved. The template receives the current OrgChartNode as its implicit context. The collapse indicator is customized with the #togglericon template, which receives the current expanded state — shown here with the plus and minus icons.

```typescript
import { Component } from '@angular/core';
import { OrganizationChartModule } from 'primeng/organizationchart';
import { Minus } from '@primeicons/angular/minus';
import { Plus } from '@primeicons/angular/plus';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <div class="flex items-center justify-center overflow-x-auto">
            <p-organization-chart [value]="data" [collapsible]="true" [(collapsedKeys)]="collapsedKeys">
                <ng-template #node let-node>
                    <div class="flex items-center gap-2">
                        <div class="size-9 rounded-md flex items-center justify-center shrink-0" [class]="node.data.accent">
                            <svg [pIcon]="node.data.icon" [size]="16" />
                        </div>
                        <div class="flex flex-col items-start gap-0.5">
                            <div class="text-sm font-semibold leading-none">{{ node.data.label }}</div>
                            <div class="text-xs leading-none opacity-75">{{ node.data.description }}</div>
                        </div>
                    </div>
                </ng-template>
                <ng-template #togglericon let-expanded>
                    @if (expanded) {
                        <svg data-p-icon="minus" />
                    } @else {
                        <svg data-p-icon="plus" />
                    }
                </ng-template>
            </p-organization-chart>
        </div>
    `,
    standalone: true,
    imports: [OrganizationChartModule, Minus, Plus, PIcon]
})
export class OrganizationChartContentDemo {
    collapsedKeys: OrganizationChartNodeKeys = {};

    data: OrgChartNode[] = [
        {
            key: '0',
            data: {
                label: 'AWS Cloud',
                description: 'us-east-1',
                icon: 'cloud',
                accent: 'bg-orange-500/10 text-orange-500'
            },
            children: [
                {
                    key: '0_0',
                    data: {
                        label: 'Compute',
                        description: 'Workloads & runtime',
                        icon: 'server',
                        accent: 'bg-sky-500/10 text-sky-500'
                    },
                    children: [
                        {
                            key: '0_0_0',
                            data: {
                                label: 'EC2',
                                description: 'Virtual servers',
                                icon: 'server',
                                accent: 'bg-sky-500/10 text-sky-500'
                            }
                        },
                        {
                            key: '0_0_1',
                            data: {
                                label: 'Lambda',
                                description: 'Serverless functions',
                                icon: 'bolt',
                                accent: 'bg-sky-500/10 text-sky-500'
                            }
                        }
                    ]
                },
                {
                    key: '0_1',
                    data: {
                        label: 'Storage',
                        description: 'Data persistence',
                        icon: 'database',
                        accent: 'bg-emerald-500/10 text-emerald-500'
                    },
                    children: [
                        {
                            key: '0_1_0',
                            data: {
                                label: 'S3',
                                description: 'Object storage',
                                icon: 'box',
                                accent: 'bg-emerald-500/10 text-emerald-500'
                            }
                        },
                        {
                            key: '0_1_1',
                            data: {
                                label: 'RDS',
                                description: 'Managed databases',
                                icon: 'database',
                                accent: 'bg-emerald-500/10 text-emerald-500'
                            }
                        }
                    ]
                },
                {
                    key: '0_2',
                    data: {
                        label: 'Networking',
                        description: 'Edge & access',
                        icon: 'globe',
                        accent: 'bg-violet-500/10 text-violet-500'
                    },
                    children: [
                        {
                            key: '0_2_0',
                            data: {
                                label: 'CloudFront',
                                description: 'Global CDN',
                                icon: 'globe',
                                accent: 'bg-violet-500/10 text-violet-500'
                            }
                        },
                        {
                            key: '0_2_1',
                            data: {
                                label: 'IAM',
                                description: 'Access policies',
                                icon: 'shield',
                                accent: 'bg-violet-500/10 text-violet-500'
                            }
                        }
                    ]
                }
            ]
        }
    ];
}
```

## Controlled

Control the collapsed state with collapsedKeys . A key present with true is collapsed; absent keys stay expanded.

```typescript
import { Component } from '@angular/core';
import { OrganizationChartModule } from 'primeng/organizationchart';
import { ButtonModule } from 'primeng/button';
import { Product } from '@/domain/product';
import { Minus } from '@primeicons/angular/minus';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4 overflow-x-auto">
            <div class="flex gap-2">
                <button type="button" pButton size="small" severity="secondary" (click)="expandAll()">Expand all</button>
                <button type="button" pButton size="small" severity="secondary" (click)="collapseAll()">Collapse all</button>
            </div>
            <p-organization-chart [value]="data" [collapsible]="true" [(collapsedKeys)]="collapsedKeys">
                <ng-template #togglericon let-expanded>
                    @if (expanded) {
                        <svg data-p-icon="minus" />
                    } @else {
                        <svg data-p-icon="plus" />
                    }
                </ng-template>
            </p-organization-chart>
        </div>
    `,
    standalone: true,
    imports: [OrganizationChartModule, ButtonModule, Minus, Plus]
})
export class OrganizationChartControlledDemo {
    collapsedKeys: OrganizationChartNodeKeys = {};

    data: OrgChartNode[] = [
        {
            key: '0',
            label: 'Founder',
            children: [
                {
                    key: '0-0',
                    label: 'Product Lead',
                    children: [
                        {
                            key: '0-0-0',
                            label: 'UX/UI Designer'
                        },
                        {
                            key: '0-0-1',
                            label: 'Product Manager'
                        }
                    ]
                },
                {
                    key: '0-1',
                    label: 'Engineering Lead',
                    children: [
                        {
                            key: '0-1-0',
                            label: 'Frontend Developer'
                        },
                        {
                            key: '0-1-1',
                            label: 'Backend Developer'
                        }
                    ]
                }
            ]
        }
    ];

    expandAll() {
        this.collapsedKeys = {};
    }

    collapseAll() {
        this.collapsedKeys = { '0': true };
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { OrganizationChartModule } from 'primeng/organizationchart';
import { Product } from '@/domain/product';

@Component({
    template: `
        <div class="flex items-center justify-center overflow-x-auto">
            <p-organization-chart [value]="data">
                <ng-template #node let-node>
                    <div class="flex items-center gap-2">
                        <p-avatar [image]="node.data.avatar" shape="circle" size="normal" />
                        <div class="flex flex-col items-start gap-0.5">
                            <div class="text-sm font-semibold leading-none">{{ node.data.name }}</div>
                            <div class="text-xs leading-none opacity-75">{{ node.data.role }}</div>
                        </div>
                    </div>
                </ng-template>
            </p-organization-chart>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, OrganizationChartModule]
})
export class OrganizationChartPreviewDemo {
    data: OrgChartNode[] = [
        {
            expanded: true,
            data: {
                name: 'Amy Elsner',
                role: 'Founder & CEO',
                avatar: 'https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png',
                initials: 'AE'
            },
            children: [
                {
                    expanded: true,
                    data: {
                        name: 'Asiya Javayant',
                        role: 'Product Lead',
                        avatar: 'https://primefaces.org/cdn/primevue/images/avatar/asiyajavayant.png',
                        initials: 'AJ'
                    },
                    children: [
                        {
                            data: {
                                name: 'Anna Fali',
                                role: 'UX Designer',
                                avatar: 'https://primefaces.org/cdn/primevue/images/avatar/annafali.png',
                                initials: 'AF'
                            }
                        },
                        {
                            data: {
                                name: 'Bernardo Dominic',
                                role: 'Product Manager',
                                avatar: 'https://primefaces.org/cdn/primevue/images/avatar/bernardodominic.png',
                                initials: 'BD'
                            }
                        }
                    ]
                },
                {
                    expanded: true,
                    data: {
                        name: 'Onyama Limba',
                        role: 'Engineering Lead',
                        avatar: 'https://primefaces.org/cdn/primevue/images/avatar/onyamalimba.png',
                        initials: 'OL'
                    },
                    children: [
                        {
                            data: {
                                name: 'Elwin Sharvill',
                                role: 'Frontend Engineer',
                                avatar: 'https://primefaces.org/cdn/primevue/images/avatar/elwinsharvill.png',
                                initials: 'ES'
                            }
                        },
                        {
                            data: {
                                name: 'Stephen Shaw',
                                role: 'Backend Engineer',
                                avatar: 'https://primefaces.org/cdn/primevue/images/avatar/stephenshaw.png',
                                initials: 'SS'
                            }
                        }
                    ]
                }
            ]
        }
    ];
}
```

## Organization Chart

OrganizationChart visualizes hierarchical organization data.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, OrganizationChartPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | OrgChartNode<any>[] \| undefined | - | An array of nested OrgChartNodes. |
| selectionMode | OrganizationChartSelectionMode \| null \| undefined | - | Defines the selection mode. |
| collapsible | boolean | false | Whether the nodes can be expanded or toggled. |
| selectable | boolean | true | Whether the nodes can be selected when a  `selectionMode`  is defined. Can be overridden per node with  `OrgChartNode.selectable` . |
| gap | number \| [number, number] | - | Defines the gap between nodes. Can be a single number or a tuple [gapX, gapY]. |
| selectionKeys | OrganizationChartNodeKeys \| null | - | A map of node keys whose values are  `true`  for the selected nodes. Two-way bindable. When omitted, it is seeded from each node's  `selectedByDefault`  flag. |
| collapsedKeys | OrganizationChartNodeKeys \| null | - | A map of node keys whose values are  `true`  for the collapsed nodes. Two-way bindable. When omitted, it is seeded from each node's  `collapsedByDefault`  flag. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onNodeSelect | event: OrganizationChartNodeSelectEvent | Callback to invoke when a node is selected. |
| onNodeUnselect | event: OrganizationChartNodeUnSelectEvent | Callback to invoke when a node is unselected. |
| onNodeExpand | event: OrganizationChartNodeExpandEvent | Callback to invoke when a node is expanded. |
| onNodeCollapse | event: OrganizationChartNodeCollapseEvent | Callback to invoke when a node is collapsed. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| togglericon | TemplateRef<any> \| undefined | Custom toggler icon template. |
| collapsebutton | TemplateRef<any> \| undefined | Custom collapse button template. |
| checkboxicon | TemplateRef<any> \| undefined | Custom checkbox icon template for  `checkbox`  selection mode. |
| node | TemplateRef<any> \| undefined | Custom node template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| subtree | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the subtree's DOM element. |
| tree | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the tree's DOM element. |
| node | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the node's DOM element. |
| nodeContent | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the node content's DOM element. |
| label | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the node label's DOM element. |
| pcCheckbox | CheckboxPassThrough | Used to pass attributes to the Checkbox component in  `checkbox`  selection mode. |
| collapseButton | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the collapse button's DOM element. |
| collapseButtonIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the collapse button icon's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-organizationchart | Class name of the root element |
| p-organizationchart-subtree | Class name of the subtree element |
| p-organizationchart-tree | Class name of the tree element |
| p-organizationchart-node | Class name of the node element |
| p-organizationchart-node-content | Class name of the node content element |
| p-organizationchart-node-label | Class name of the node label element |
| p-organizationchart-node-toggle-button | Class name of the collapse button element |
| p-organizationchart-node-toggle-button-icon | Class name of the collapse button icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| organizationchart.gutter | --p-organizationchart-gutter | Gutter of root |
| organizationchart.transition.duration | --p-organizationchart-transition-duration | Transition duration of root |
| organizationchart.node.background | --p-organizationchart-node-background | Background of node |
| organizationchart.node.hover.background | --p-organizationchart-node-hover-background | Hover background of node |
| organizationchart.node.selected.background | --p-organizationchart-node-selected-background | Selected background of node |
| organizationchart.node.border.color | --p-organizationchart-node-border-color | Border color of node |
| organizationchart.node.color | --p-organizationchart-node-color | Color of node |
| organizationchart.node.selected.color | --p-organizationchart-node-selected-color | Selected color of node |
| organizationchart.node.hover.color | --p-organizationchart-node-hover-color | Hover color of node |
| organizationchart.node.padding | --p-organizationchart-node-padding | Padding of node |
| organizationchart.node.toggleable.padding | --p-organizationchart-node-toggleable-padding | Toggleable padding of node |
| organizationchart.node.border.radius | --p-organizationchart-node-border-radius | Border radius of node |
| organizationchart.node.font.size | --p-organizationchart-node-font-size | Font size of node |
| organizationchart.node.font.weight | --p-organizationchart-node-font-weight | Font weight of node |
| organizationchart.node.focus.ring.width | --p-organizationchart-node-focus-ring-width | Focus ring width of node |
| organizationchart.node.focus.ring.style | --p-organizationchart-node-focus-ring-style | Focus ring style of node |
| organizationchart.node.focus.ring.color | --p-organizationchart-node-focus-ring-color | Focus ring color of node |
| organizationchart.node.focus.ring.offset | --p-organizationchart-node-focus-ring-offset | Focus ring offset of node |
| organizationchart.node.focus.ring.shadow | --p-organizationchart-node-focus-ring-shadow | Focus ring shadow of node |
| organizationchart.node.toggle.button.background | --p-organizationchart-node-toggle-button-background | Background of node toggle button |
| organizationchart.node.toggle.button.hover.background | --p-organizationchart-node-toggle-button-hover-background | Hover background of node toggle button |
| organizationchart.node.toggle.button.border.color | --p-organizationchart-node-toggle-button-border-color | Border color of node toggle button |
| organizationchart.node.toggle.button.color | --p-organizationchart-node-toggle-button-color | Color of node toggle button |
| organizationchart.node.toggle.button.hover.color | --p-organizationchart-node-toggle-button-hover-color | Hover color of node toggle button |
| organizationchart.node.toggle.button.size | --p-organizationchart-node-toggle-button-size | Size of node toggle button |
| organizationchart.node.toggle.button.border.radius | --p-organizationchart-node-toggle-button-border-radius | Border radius of node toggle button |
| organizationchart.node.toggle.button.focus.ring.width | --p-organizationchart-node-toggle-button-focus-ring-width | Focus ring width of node toggle button |
| organizationchart.node.toggle.button.focus.ring.style | --p-organizationchart-node-toggle-button-focus-ring-style | Focus ring style of node toggle button |
| organizationchart.node.toggle.button.focus.ring.color | --p-organizationchart-node-toggle-button-focus-ring-color | Focus ring color of node toggle button |
| organizationchart.node.toggle.button.focus.ring.offset | --p-organizationchart-node-toggle-button-focus-ring-offset | Focus ring offset of node toggle button |
| organizationchart.node.toggle.button.focus.ring.shadow | --p-organizationchart-node-toggle-button-focus-ring-shadow | Focus ring shadow of node toggle button |
| organizationchart.node.toggle.button.icon.size | --p-organizationchart-node-toggle-button-icon-size | Icon size of node toggle button |
| organizationchart.connector.color | --p-organizationchart-connector-color | Color of connector |
| organizationchart.connector.border.radius | --p-organizationchart-connector-border-radius | Border radius of connector |
| organizationchart.connector.height | --p-organizationchart-connector-height | Height of connector |
