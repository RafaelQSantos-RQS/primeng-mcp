---
title: "Angular TreeTable Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/treetable.md"
description: "TreeTable is used to display hierarchical data in tabular format."
---

# Angular TreeTable Component

TreeTable is used to display hierarchical data in tabular format.

## Accessibility

Screen Reader Default role of the table is table . Header, body and footer elements use rowgroup , rows use row role, header cells have columnheader and body cells use cell roles. Sortable headers utilizer aria-sort attribute either set to "ascending" or "descending". Row elements manage aria-expanded for state and aria-level attribute to define the hierachy by ttRow directive. Table rows and table cells should be specified by users using the aria-posinset , aria-setsize , aria-label , and aria-describedby attributes, as they are determined through templating. When selection is enabled, ttSelectableRow directive sets aria-selected to true on a row. In checkbox mode, the built-in checkbox component use checkbox role with aria-checked state attribute. Editable cells use custom templating so you need to manage aria roles and attributes manually if required. Paginator is a standalone component used inside the TreeTable, refer to the paginator for more information about the accessibility features. Sortable Headers Keyboard Support Key Function tab Moves through the headers. enter Sorts the column. space Sorts the column. Keyboard Support Key Function tab Moves focus to the first selected node when focus enters the component, if there is none then first element receives the focus. If focus is already inside the component, moves focus to the next focusable element in the page tab sequence. shift + tab Moves focus to the last selected node when focus enters the component, if there is none then first element receives the focus. If focus is already inside the component, moves focus to the previous focusable element in the page tab sequence. enter Selects the focused treenode. space Selects the focused treenode. down arrow Moves focus to the next treenode. up arrow Moves focus to the previous treenode. right arrow If node is closed, opens the node otherwise moves focus to the first child node. left arrow If node is open, closes the node otherwise moves focus to the parent node.

## Advanced

Sort, filter (per-column + global), and cell editing composed in a single tree.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { InputTextModule } from 'primeng/inputtext';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Search } from '@primeicons/angular/search';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <div class="mb-3 flex justify-end">
            <p-iconfield class="w-full sm:max-w-xs">
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input type="text" pInputText pSize="small" placeholder="Keyword search" (input)="tt.filterGlobal($event.target.value, 'contains')" class="w-full" />
            </p-iconfield>
        </div>
        <p-treetable #tt [value]="files" [globalFilterFields]="['name', 'type']" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header>
                <tr>
                    <th ttSortableColumn="name">Name <p-treetable-sort-icon field="name" /></th>
                    <th ttSortableColumn="size">Size <p-treetable-sort-icon field="size" /></th>
                    <th ttSortableColumn="type">Type <p-treetable-sort-icon field="type" /></th>
                </tr>
                <tr>
                    <th>
                        <input pInputText pSize="small" type="text" placeholder="Search name..." (input)="tt.filter($event.target.value, 'name', 'contains')" class="w-full" />
                    </th>
                    <th></th>
                    <th>
                        <p-select [options]="typeOptions" (onChange)="tt.filter($event.value, 'type', 'equals')" placeholder="Any" [showClear]="true" size="small" class="w-full" />
                    </th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td ttEditableColumn>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <p-treetable-cell-editor>
                                <ng-template #input>
                                    <input pInputText pSize="small" type="text" [(ngModel)]="rowData.name" class="w-full" />
                                </ng-template>
                                <ng-template #output>
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                                </ng-template>
                            </p-treetable-cell-editor>
                        </div>
                    </td>
                    <td ttEditableColumn>
                        <p-treetable-cell-editor>
                            <ng-template #input>
                                <input pInputText pSize="small" type="text" [(ngModel)]="rowData.size" class="w-full" />
                            </ng-template>
                            <ng-template #output>
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                            </ng-template>
                        </p-treetable-cell-editor>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
            <ng-template #emptymessage>
                <tr>
                    <td colspan="3">No data found.</td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [SelectModule, IconFieldModule, InputIconModule, TagModule, TreeTableModule, InputTextModule, FormsModule, Search, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableAdvancedDemo implements OnInit {
    files!: TreeNode[];

    typeOptions = ['Folder', 'Document', 'Text', 'Picture', 'Video'];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 6);
            this.files[0].expanded = true;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Basic

TreeTable requires a collection of TreeNode instances as a value with ttRow rows and the toggler placed on the column that should expand or collapse a node.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-treetable [value]="nodes" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th style="width: 34%">Name</th>
                    <th style="width: 33%">Size</th>
                    <th style="width: 33%">Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="inline-flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <span>{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>{{ rowData.size }}</td>
                    <td>{{ rowData.type }}</td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableBasicDemo implements OnInit {
    nodes!: TreeNode[];

    ngOnInit() {
        this.nodes = [
            {
                key: '0',
                data: { name: 'Documents', size: '75kb', type: 'Folder' },
                children: [
                    {
                        key: '0-0',
                        data: { name: 'Work', size: '55kb', type: 'Folder' },
                        children: [
                            { key: '0-0-0', data: { name: 'Expenses.doc', size: '30kb', type: 'Document' } },
                            { key: '0-0-1', data: { name: 'Resume.doc', size: '25kb', type: 'Document' } }
                        ]
                    },
                    {
                        key: '0-1',
                        data: { name: 'Home', size: '20kb', type: 'Folder' },
                        children: [{ key: '0-1-0', data: { name: 'Invoices.txt', size: '20kb', type: 'Text' } }]
                    }
                ]
            },
            {
                key: '1',
                data: { name: 'Pictures', size: '150kb', type: 'Folder' },
                children: [
                    { key: '1-0', data: { name: 'barcelona.jpg', size: '90kb', type: 'Picture' } },
                    { key: '1-1', data: { name: 'primeui.png', size: '30kb', type: 'Picture' } },
                    { key: '1-2', data: { name: 'optimus.jpg', size: '30kb', type: 'Picture' } }
                ]
            },
            {
                key: '2',
                data: { name: 'Movies', size: '1500kb', type: 'Folder' },
                children: [
                    {
                        key: '2-0',
                        data: { name: 'Al Pacino', size: '1000kb', type: 'Folder' },
                        children: [
                            { key: '2-0-0', data: { name: 'Scarface', size: '500kb', type: 'Video' } },
                            { key: '2-0-1', data: { name: 'Serpico', size: '500kb', type: 'Video' } }
                        ]
                    },
                    {
                        key: '2-1',
                        data: { name: 'Robert De Niro', size: '500kb', type: 'Folder' },
                        children: [
                            { key: '2-1-0', data: { name: 'Goodfellas', size: '250kb', type: 'Video' } },
                            { key: '2-1-1', data: { name: 'Untouchables', size: '250kb', type: 'Video' } }
                        ]
                    }
                ]
            }
        ];
    }
}
```

## Column Group

Columns can be grouped at header and footer using rowspan and colspan properties.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';
import { Product } from '@/domain/product';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <p-treetable [value]="sales" showGridlines [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th rowspan="2">Product</th>
                    <th colspan="2" style="text-align: center">Sale Rate</th>
                    <th colspan="2" style="text-align: center">Profits</th>
                </tr>
                <tr>
                    <th>Last Year</th>
                    <th>This Year</th>
                    <th>Last Year</th>
                    <th>This Year</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <span [class]="rowNode.node.children?.length ? 'font-medium' : ''">{{ rowData.product }}</span>
                        </div>
                    </td>
                    <td>{{ rowData.lastYearSale }}%</td>
                    <td>
                        <div class="inline-flex items-center gap-1">
                            <span>{{ rowData.thisYearSale }}%</span>
                            @if (rowData.thisYearSale >= rowData.lastYearSale) {
                                <svg data-p-icon="arrow-up" [size]="12" class="text-green-500" />
                            } @else {
                                <svg data-p-icon="arrow-down" [size]="12" class="text-red-500" />
                            }
                        </div>
                    </td>
                    <td>{{ '$' + rowData.lastYearProfit.toLocaleString() }}</td>
                    <td>
                        <p-tag [value]="'$' + rowData.thisYearProfit.toLocaleString()" [severity]="rowData.thisYearProfit >= rowData.lastYearProfit ? 'success' : 'danger'" />
                    </td>
                </tr>
            </ng-template>
            <ng-template #footer>
                <tr>
                    <td colspan="3" style="text-align: right">
                        <span class="font-semibold">Totals</span>
                    </td>
                    <td>
                        <span class="font-semibold">{{ '$' + lastYearTotal.toLocaleString() }}</span>
                    </td>
                    <td>
                        <span class="font-semibold">{{ '$' + thisYearTotal.toLocaleString() }}</span>
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, ArrowUp, ArrowDown],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableColumnGroupDemo implements OnInit {
    sales!: TreeNode[];

    lastYearTotal: number = 0;

    thisYearTotal: number = 0;

    ngOnInit() {
        this.sales = [
            {
                key: 'europe',
                expanded: true,
                data: { product: 'Europe', lastYearSale: 60, thisYearSale: 45, lastYearProfit: 766_537, thisYearProfit: 659_964 },
                children: [
                    { key: 'e-1', data: { product: 'Bamboo Watch', lastYearSale: 51, thisYearSale: 40, lastYearProfit: 54_406, thisYearProfit: 43_342 } },
                    { key: 'e-2', data: { product: 'Black Watch', lastYearSale: 83, thisYearSale: 9, lastYearProfit: 423_132, thisYearProfit: 312_122 } },
                    { key: 'e-3', data: { product: 'Blue T-Shirt', lastYearSale: 49, thisYearSale: 22, lastYearProfit: 288_999, thisYearProfit: 304_500 } }
                ]
            },
            {
                key: 'americas',
                data: { product: 'Americas', lastYearSale: 28, thisYearSale: 42, lastYearProfit: 655_563, thisYearProfit: 508_832 },
                children: [
                    { key: 'a-1', data: { product: 'Blue Band', lastYearSale: 38, thisYearSale: 5, lastYearProfit: 12_321, thisYearProfit: 8_500 } },
                    { key: 'a-2', data: { product: 'Bracelet', lastYearSale: 17, thisYearSale: 79, lastYearProfit: 643_242, thisYearProfit: 500_332 } }
                ]
            }
        ];
        this.lastYearTotal = this.sales.reduce((sum, n) => sum + n.data.lastYearProfit, 0);
        this.thisYearTotal = this.sales.reduce((sum, n) => sum + n.data.thisYearProfit, 0);
    }
}
```

## columnresizeexpand-doc

Setting columnResizeMode as expand changes the table width as well.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [resizableColumns]="true" columnResizeMode="expand" showGridlines>
            <ng-template #header>
                <tr>
                    <th ttResizableColumn>Name</th>
                    <th ttResizableColumn>Size</th>
                    <th ttResizableColumn>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableColumnResizeExpandDemo implements OnInit {
    files!: TreeNode[];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## columnresizefit-doc

Columns can be resized with drag and drop when resizableColumns is enabled. Default resize mode is fit that does not change the overall table width.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [resizableColumns]="true" [tableStyle]="{ 'min-width': '50rem' }" showGridlines>
            <ng-template #header>
                <tr>
                    <th ttResizableColumn>Name</th>
                    <th ttResizableColumn>Size</th>
                    <th ttResizableColumn>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableColumnResizeFitDemo implements OnInit {
    files!: TreeNode[];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## columnresizescrollable-doc

To utilize the column resize modes with a scrollable TreeTable, a colgroup template must be defined. The default value of scrollHeight is "flex," it can also be set as a string value.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTable, TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [resizableColumns]="true" [scrollable]="true" scrollHeight="200px" showGridlines>
            <ng-template #colgroup let-columns>
                <colgroup>
                    @for (col of columns; track col) {
                        <col />
                    }
                </colgroup>
            </ng-template>
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th ttResizableColumn>{{ col.header }}</th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableColumnResizeScrollableDemo implements OnInit {
    files!: TreeNode[];

    cols!: Column[];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Column Toggle

Show/hide columns dynamically.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { PopoverModule } from 'primeng/popover';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Cog } from '@primeicons/angular/cog';
import { Refresh } from '@primeicons/angular/refresh';
import { Bars } from '@primeicons/angular/bars';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <div class="mb-3 flex items-center justify-end">
            <button type="button" pButton variant="outlined" severity="secondary" size="small" (click)="op.toggle($event)">
                <svg data-p-icon="cog" />
                Columns
            </button>
            <p-popover #op styleClass="w-72 p-0!">
                <ng-template #content>
                    <div class="flex items-center justify-between gap-2 px-4 py-3 border-b border-surface-200 dark:border-surface-700">
                        <span class="text-sm font-semibold">Columns</span>
                        <button type="button" pButton variant="text" size="small" severity="secondary" (click)="reset()">
                            <svg data-p-icon="refresh" />
                            Reset
                        </button>
                    </div>
                    <div class="py-2 max-h-80 overflow-auto" cdkDropList (cdkDropListDropped)="drop($event)">
                        @for (col of cols; track col.field) {
                            <div class="flex items-center gap-2 px-3 py-1.5 mx-1 rounded-md cursor-move select-none hover:bg-surface-100 dark:hover:bg-surface-800" cdkDrag>
                                <svg data-p-icon="bars" [size]="14" class="text-surface-400 dark:text-surface-500" />
                                <p-checkbox [(ngModel)]="visibleFields" [value]="col.field" [inputId]="col.field" />
                                <label [for]="col.field" class="text-sm cursor-pointer">{{ col.header }}</label>
                            </div>
                        }
                    </div>
                </ng-template>
            </p-popover>
        </div>
        <p-treetable [value]="files" [columns]="visibleColumns" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col.field) {
                        <th>{{ col.header }}</th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else if (col.field === 'key') {
                                <code class="text-xs text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</code>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [CheckboxModule, PopoverModule, TagModule, TreeTableModule, ButtonModule, FormsModule, Cog, Refresh, Bars, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableColumnToggleDemo implements OnInit {
    files!: TreeNode[];

    cols = [...initialColumns];

    visibleFields = [...defaultVisibleFields];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = this.enrichNodes(files.slice(0, 5));
            this.files[0].expanded = true;
        });
    }

    get visibleColumns(): Column[] {
        return this.cols.filter((col) => this.visibleFields.includes(col.field));
    }

    enrichNodes(nodes: TreeNode[], level = 0): TreeNode[] {
        return nodes.map((node) => ({
            ...node,
            data: { ...node.data, key: node.key, level, hasChildren: node.children?.length ? 'Yes' : 'No' },
            children: node.children ? this.enrichNodes(node.children, level + 1) : undefined
        }));
    }

    drop(event: CdkDragDrop<Column[]>) {
        moveItemInArray(this.cols, event.previousIndex, event.currentIndex);
    }

    reset() {
        this.cols = [...initialColumns];
        this.visibleFields = [...defaultVisibleFields];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Conditional Style

Particular rows and cells can be styled based on conditions. The ngClass receives a row data as a parameter to return a style class for a row whereas cells are customized using the body template.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th>
                            {{ col.header }}
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode" [ngClass]="{ 'bg-surface-100! dark:bg-surface-800!': rowData.size.endsWith('kb') }">
                    @for (col of columns; let first = $first; track col) {
                        <td [class]="{ 'line-through': col.field === 'size' && rowData.size.endsWith('kb') }">
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode"></p-treetable-toggler>
                                    <span>{{ rowData[col.field] }}</span>
                                </div>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableConditionalStyleDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }
}
```

## Context Menu

TreeTable has exclusive integration with contextmenu component. In order to attach a menu to a table, add ttContextMenuRow directive to the rows that can be selected with context menu, define a local template variable for the menu and bind it to the contextMenu property of the table. This enables displaying the menu whenever a row is right clicked. A separate contextMenuSelection property is used to get a hold of the right clicked row. For dynamic columns, setting ttContextMenuRowDisabled property as true disables context menu for that particular row.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TreeTable, TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode, MenuItem, MessageService } from 'primeng/api';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" dataKey="name" [(contextMenuSelection)]="selectedNode" [contextMenu]="cm" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th>
                            {{ col.header }}
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode" [ttContextMenuRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode"></p-treetable-toggler>
                                    <span>{{ rowData[col.field] }}</span>
                                </div>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    providers: [NodeService, MessageService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableContextMenuDemo implements OnInit {
    private nodeService = inject(NodeService);

    private messageService = inject(MessageService);

    files!: TreeNode[];

    selectedNode!: TreeNode;

    cols!: Column[];

    items!: MenuItem[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
        this.items = [
            { label: 'View', icon: 'pi pi-search', command: () => this.viewFile(this.selectedNode) },
            { label: 'Toggle', icon: 'pi pi-sort', command: () => this.toggleFile(this.selectedNode) }
        ];
    }

    viewFile(node: any) {
        this.messageService.add({ severity: 'info', summary: 'File Selected', detail: node.data.name + ' - ' + node.data.size });
    }

    toggleFile(node: any) {
        node.expanded = !node.expanded;
        this.files = [...this.files];
    }
}
```

## Controlled

Expansion state is controlled with expandedKeys property.

```typescript
import { Component, OnInit, inject, ChangeDetectorRef, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableControlledDemo implements OnInit {
    private nodeService = inject(NodeService);

    private cd = inject(ChangeDetectorRef);

    files!: TreeNode[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => {
            this.files = files.slice(0, 5);
        });
    }

    toggleApplications() {
        if (this.files && this.files.length > 0) {
            const newFiles = [...this.files];

            newFiles[0] = { ...newFiles[0], expanded: !newFiles[0].expanded };
            this.files = newFiles;
        }
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Dynamic Columns

Columns can be created programmatically.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th>
                            {{ col.header }}
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode"></p-treetable-toggler>
                                    <span>{{ rowData[col.field] }}</span>
                                </div>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableDynamicColumnsDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }
}
```

## edit-doc

Incell editing is enabled by defining input elements with treeTableCellEditor .

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { InputTextModule } from 'primeng/inputtext';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td ttEditableColumn>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <p-treetable-cell-editor class="flex-1">
                                <ng-template #input>
                                    <input pInputText type="text" [(ngModel)]="rowData.name" pSize="small" class="w-full" />
                                </ng-template>
                                <ng-template #output>
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                                </ng-template>
                            </p-treetable-cell-editor>
                        </div>
                    </td>
                    <td ttEditableColumn>
                        <p-treetable-cell-editor>
                            <ng-template #input>
                                <input pInputText type="text" [(ngModel)]="rowData.size" pSize="small" class="w-full" />
                            </ng-template>
                            <ng-template #output>
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                            </ng-template>
                        </p-treetable-cell-editor>
                    </td>
                    <td ttEditableColumn>
                        <p-treetable-cell-editor>
                            <ng-template #input>
                                <input pInputText type="text" [(ngModel)]="rowData.type" pSize="small" class="w-full" />
                            </ng-template>
                            <ng-template #output>
                                <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                            </ng-template>
                        </p-treetable-cell-editor>
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, InputTextModule, FormsModule, Folder, Image, File],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableEditDemo implements OnInit {
    files!: TreeNode[];

    ngOnInit() {
        this.files = [
            {
                key: '0',
                expanded: true,
                data: { name: 'Documents', size: '75kb', type: 'Folder' },
                children: [
                    {
                        key: '0-0',
                        expanded: true,
                        data: { name: 'Work', size: '55kb', type: 'Folder' },
                        children: [
                            { key: '0-0-0', data: { name: 'Expenses.doc', size: '30kb', type: 'Document' } },
                            { key: '0-0-1', data: { name: 'Resume.doc', size: '25kb', type: 'Document' } }
                        ]
                    },
                    {
                        key: '0-1',
                        data: { name: 'Home', size: '20kb', type: 'Folder' },
                        children: [{ key: '0-1-0', data: { name: 'Invoices.txt', size: '20kb', type: 'Text' } }]
                    }
                ]
            },
            {
                key: '1',
                data: { name: 'Pictures', size: '150kb', type: 'Folder' },
                children: [
                    { key: '1-0', data: { name: 'barcelona.jpg', size: '90kb', type: 'Picture' } },
                    { key: '1-1', data: { name: 'primeui.png', size: '30kb', type: 'Picture' } },
                    { key: '1-2', data: { name: 'optimus.jpg', size: '30kb', type: 'Picture' } }
                ]
            }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Empty State

Use the emptymessage template for a custom empty-state view.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { Folder } from '@primeicons/angular/folder';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <p-treetable [value]="[]" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header>
                <tr>
                    <th style="width: 50%">Name</th>
                    <th style="width: 20%">Size</th>
                    <th style="width: 30%">Type</th>
                </tr>
            </ng-template>
            <ng-template #emptymessage>
                <tr>
                    <td colspan="3">
                        <div class="flex flex-col items-center justify-center gap-3 py-10 text-center">
                            <div class="w-14 h-14 rounded-full bg-surface-100 dark:bg-surface-800 flex items-center justify-center">
                                <svg data-p-icon="folder" [size]="28" class="text-surface-400 dark:text-surface-500" />
                            </div>
                            <div>
                                <p class="m-0 font-semibold text-surface-900 dark:text-surface-0">No folders yet</p>
                                <p class="mt-1 text-sm text-surface-500 dark:text-surface-400">Create your first folder to start building a tree.</p>
                            </div>
                            <button type="button" pButton size="small">
                                <svg data-p-icon="plus" />
                                New Folder
                            </button>
                        </div>
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule, ButtonModule, Folder, Plus],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableEmptyStateDemo {}
```

## filterbasic-doc

Per-column row filters with a global keyword search. The filterMode controls whether matching is lenient (includes all descendants) or strict (filters every level independently).

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { SelectModule } from 'primeng/select';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { InputTextModule } from 'primeng/inputtext';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Search } from '@primeicons/angular/search';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <div class="mb-3 flex justify-end">
            <p-iconfield class="w-full sm:max-w-xs">
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input type="text" pInputText pSize="small" placeholder="Keyword search" (input)="tt.filterGlobal($event.target.value, 'contains')" class="w-full" />
            </p-iconfield>
        </div>
        <p-treetable #tt [value]="files" [globalFilterFields]="['name', 'type']" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
                <tr>
                    <th>
                        <input pInputText pSize="small" type="text" placeholder="Search name..." (input)="tt.filter($event.target.value, 'name', 'contains')" class="w-full" />
                    </th>
                    <th></th>
                    <th>
                        <p-select [options]="typeOptions" (onChange)="tt.filter($event.value, 'type', 'equals')" placeholder="Any" [showClear]="true" size="small" class="w-full" />
                    </th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
            <ng-template #emptymessage>
                <tr>
                    <td colspan="3">No data found.</td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [SelectModule, IconFieldModule, InputIconModule, TagModule, TreeTableModule, InputTextModule, Search, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableFilterBasicDemo implements OnInit {
    files!: TreeNode[];

    typeOptions = ['Folder', 'Document', 'Text', 'Picture', 'Video'];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => (this.files = files));
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## flexiblescroll-doc

Flex scroll feature makes the scrollable viewport section dynamic instead of a fixed value so that it can grow or shrink relative to the parent size of the table. Click the button below to display a maximizable Dialog where data viewport adjusts itself according to the size changes.

```typescript
import { Component, OnInit, inject, ChangeDetectorRef, ChangeDetectionStrategy } from '@angular/core';
import { Dialog, DialogModule } from 'primeng/dialog';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <button pButton (click)="dialogVisible = true"><i class="pi pi-external-link"></i> Show</button>
        <p-dialog [(visible)]="dialogVisible" header="Flex Scroll" [style]="{ width: '75vw' }" maximizable modal [contentStyle]="{ height: '300px' }">
            <ng-template #content>
                <p-treetable [value]="files" [scrollable]="true" scrollHeight="flex" [tableStyle]="{ 'min-width': '50rem' }">
                    <ng-template #header>
                        <tr>
                            <th>Name</th>
                            <th>Size</th>
                            <th>Type</th>
                        </tr>
                    </ng-template>
                    <ng-template #body let-rowNode let-rowData="rowData">
                        <tr [ttRow]="rowNode">
                            <td>
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                                </div>
                            </td>
                            <td>
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                            </td>
                            <td>
                                <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                            </td>
                        </tr>
                    </ng-template>
                </p-treetable>
            </ng-template>
            <ng-template #footer>
                <button pButton (click)="dialogVisible = false"><i class="pi pi-check"></i> Ok</button>
            </ng-template>
        </p-dialog>
    `,
    standalone: true,
    imports: [DialogModule, TagModule, TreeTableModule, ButtonModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableFlexibleScrollDemo implements OnInit {
    private nodeService = inject(NodeService);

    private cd = inject(ChangeDetectorRef);

    files!: TreeNode[];

    dialogVisible: boolean = false;

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => {
            this.files = files;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Grid Lines

Enable the showGridlines property to render borders between cells.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [scrollable]="true" showGridlines [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableGridLinesDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Lazy Load

Children can be loaded on demand using onNodeExpand callback to implement lazy loading.

```typescript
import { Component, OnInit, inject, ChangeDetectorRef, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-treetable [value]="files" (onNodeExpand)="onNodeExpand($event)" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <span>{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>{{ rowData.size }}</td>
                    <td>{{ rowData.type }}</td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableLazyLoadDemo implements OnInit {
    files!: TreeNode[];

    private cd = inject(ChangeDetectorRef);

    ngOnInit() {
        this.files = [
            { key: '0', data: { name: 'Applications', size: '100kb', type: 'Folder' }, leaf: false },
            { key: '1', data: { name: 'Cloud', size: '20kb', type: 'Folder' }, leaf: false },
            { key: '2', data: { name: 'Desktop', size: '150kb', type: 'Folder' }, leaf: false },
            { key: '3', data: { name: 'Documents', size: '75kb', type: 'Folder' }, leaf: false }
        ];
    }

    onNodeExpand(event: any) {
        const node = event.node;

        if (node.children) return;

        node.loading = true;
        this.files = [...this.files];
        this.cd.markForCheck();

        setTimeout(() => {
            node.children = Array.from({ length: 3 }, (_, i) => ({
                key: `${node.key}-${i}`,
                data: { name: `File ${node.key}.${i + 1}`, size: `${(i + 1) * 10}kb`, type: 'Document' },
                leaf: true
            }));
            node.loading = false;
            this.files = [...this.files];
            this.cd.markForCheck();
        }, 600);
    }
}
```

## loadingmask-doc

The loading property displays a mask layer to indicate busy state.

```typescript
import { Component, OnInit, inject, ChangeDetectorRef, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Refresh } from '@primeicons/angular/refresh';
import { Spinner } from '@primeicons/angular/spinner';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <div class="mb-3 flex items-center justify-between gap-3">
            <span class="text-sm text-surface-500 dark:text-surface-400">Click refresh to simulate a network fetch.</span>
            <button type="button" pButton size="small" [disabled]="loading" (click)="refresh()">
                <svg data-p-icon="refresh" />
                Refresh
            </button>
        </div>
        <p-treetable [value]="files" [loading]="loading" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #loadingicon>
                <div class="flex flex-col items-center gap-2 text-nowrap">
                    <svg data-p-icon="spinner" [size]="40" class="animate-spin text-primary" />
                    <span class="text-sm text-surface-600 dark:text-surface-300">Loading nodes…</span>
                </div>
            </ng-template>
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, ButtonModule, Refresh, Spinner, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableLoadingMaskDemo implements OnInit {
    files!: TreeNode[];

    loading: boolean = false;

    private nodeService = inject(NodeService);

    private cd = inject(ChangeDetectorRef);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    refresh() {
        this.loading = true;
        this.cd.markForCheck();

        setTimeout(() => {
            this.nodeService.getTreeTableNodes().then((files) => {
                this.files = files.slice(0, 5);
                this.files[0].expanded = true;
                this.loading = false;
                this.cd.markForCheck();
            });
        }, 1500);
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## loadingskeleton-doc

Skeleton placeholders are displayed during the loading process.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-treetable [value]="files" [tableStyle]="{ 'min-width': '40rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode>
                <tr [ttRow]="rowNode">
                    <td>
                        <p-skeleton styleClass="h-5 w-3/4" />
                    </td>
                    <td>
                        <p-skeleton styleClass="h-5 w-1/3" />
                    </td>
                    <td>
                        <p-skeleton styleClass="h-5 w-1/2" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [SkeletonModule, TreeTableModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableLoadingSkeletonDemo {
    files = placeholders;
}
```

## multiplepresort-doc

Apply an initial multi-column sort on mount by setting sortMode to multiple together with multiSortMeta . Each header keeps a badge with its sort order and stays interactive afterwards.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" sortMode="multiple" [multiSortMeta]="multiSortMeta" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; let first = $first; track col) {
                        <th [ttSortableColumn]="col.field" [style.width]="first ? '50%' : ''">
                            <div class="flex items-center gap-2">
                                {{ col.header }}
                                <p-treetable-sort-icon [field]="col.field" />
                            </div>
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td [style.width]="first ? '50%' : ''">
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableMultiplePresortDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    multiSortMeta: SortMeta[] = [
        { field: 'type', order: 1 },
        { field: 'name', order: 1 }
    ];

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Basic

Pagination is enabled by adding paginator property and defining rows per page.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-treetable [value]="files" [paginator]="true" [rows]="5" [rowsPerPageOptions]="[5, 10, 25]" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th style="width: 34%">Name</th>
                    <th style="width: 33%">Size</th>
                    <th style="width: 33%">Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="inline-flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <span>{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>{{ rowData.size }}</td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTablePaginatorBasicDemo implements OnInit {
    files!: TreeNode[];

    ngOnInit() {
        const files: TreeNode[] = [];
        for (let i = 0; i < 50; i++) {
            files.push({
                key: `${i}`,
                data: {
                    name: 'Item ' + i,
                    size: Math.floor(Math.random() * 1000) + 1 + 'kb',
                    type: 'Type ' + i
                },
                children: [
                    {
                        key: `${i} - 0`,
                        data: {
                            name: 'Item ' + i + ' - 0',
                            size: Math.floor(Math.random() * 1000) + 1 + 'kb',
                            type: 'Type ' + i
                        }
                    }
                ]
            });
        }
        this.files = files;
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Template

Paginator UI is customized using the paginatorleft and paginatorright templates. Each element can also be customized further with your own UI to replace the default one, refer to the Paginator component for more information about the advanced customization options.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { TreeNode } from 'primeng/api';
import { Paginator } from 'primeng/paginator';
import { Refresh } from '@primeicons/angular/refresh';
import { Download } from '@primeicons/angular/download';

@Component({
    template: `
        <p-treetable [value]="files" [paginator]="true" [rows]="5" [rowsPerPageOptions]="[5, 10, 25, 50]" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th style="width: 34%">Name</th>
                    <th style="width: 33%">Size</th>
                    <th style="width: 33%">Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="inline-flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <span>{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>{{ rowData.size }}</td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
            <ng-template #paginatorleft>
                <button pButton type="button" iconOnly text>
                    <svg data-p-icon="refresh" />
                </button>
            </ng-template>
            <ng-template #paginatorright>
                <button pButton type="button" iconOnly text>
                    <svg data-p-icon="download" />
                </button>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, ButtonModule, Refresh, Download],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTablePaginatorTemplateDemo implements OnInit {
    files!: TreeNode[];

    ngOnInit() {
        const files: TreeNode[] = [];
        for (let i = 0; i < 50; i++) {
            files.push({
                key: `${i}`,
                data: {
                    name: 'Item ' + i,
                    size: Math.floor(Math.random() * 1000) + 1 + 'kb',
                    type: 'Type ' + i
                },
                children: [
                    {
                        key: `${i} - 0`,
                        data: {
                            name: 'Item ' + i + ' - 0',
                            size: Math.floor(Math.random() * 1000) + 1 + 'kb',
                            type: 'Type ' + i
                        }
                    }
                ]
            });
        }
        this.files = files;
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Presort

Apply an initial sort on mount using sortField and sortOrder . Headers stay interactive afterwards.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" sortField="size" [sortOrder]="-1" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; let first = $first; track col) {
                        <th [ttSortableColumn]="col.field" [style.width]="first ? '50%' : ''">
                            <div class="flex items-center gap-2">
                                {{ col.header }}
                                <p-treetable-sort-icon [field]="col.field" />
                            </div>
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td [style.width]="first ? '50%' : ''">
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTablePresortDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## preview-doc

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="nodes" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTablePreviewDemo {
    nodes: TreeNode[] = [
        {
            key: '0',
            data: { name: 'Documents', size: '75kb', type: 'Folder' },
            children: [
                {
                    key: '0-0',
                    data: { name: 'Work', size: '55kb', type: 'Folder' },
                    children: [
                        { key: '0-0-0', data: { name: 'Expenses.doc', size: '30kb', type: 'Document' } },
                        { key: '0-0-1', data: { name: 'Resume.doc', size: '25kb', type: 'Document' } }
                    ]
                },
                {
                    key: '0-1',
                    data: { name: 'Home', size: '20kb', type: 'Folder' },
                    children: [{ key: '0-1-0', data: { name: 'Invoices.txt', size: '20kb', type: 'Text' } }]
                }
            ]
        },
        {
            key: '1',
            data: { name: 'Pictures', size: '150kb', type: 'Folder' },
            children: [
                { key: '1-0', data: { name: 'barcelona.jpg', size: '90kb', type: 'Picture' } },
                { key: '1-1', data: { name: 'primeui.png', size: '30kb', type: 'Picture' } },
                { key: '1-2', data: { name: 'optimus.jpg', size: '30kb', type: 'Picture' } }
            ]
        },
        {
            key: '2',
            data: { name: 'Movies', size: '1500kb', type: 'Folder' },
            children: [
                {
                    key: '2-0',
                    data: { name: 'Al Pacino', size: '1000kb', type: 'Folder' },
                    children: [
                        { key: '2-0-0', data: { name: 'Scarface', size: '500kb', type: 'Video' } },
                        { key: '2-0-1', data: { name: 'Serpico', size: '500kb', type: 'Video' } }
                    ]
                },
                {
                    key: '2-1',
                    data: { name: 'Robert De Niro', size: '500kb', type: 'Folder' },
                    children: [
                        { key: '2-1-0', data: { name: 'Goodfellas', size: '250kb', type: 'Video' } },
                        { key: '2-1-1', data: { name: 'Untouchables', size: '250kb', type: 'Video' } }
                    ]
                }
            ]
        }
    ];

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Reorder

Drag and drop column headers to reorder columns with reorderableColumns .

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [reorderableColumns]="true" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th ttReorderableColumn>
                            {{ col.header }}
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableReorderDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Row Edit

Row editing toggles the visibility of all editors in the row at once, providing save and cancel actions.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { File } from '@primeicons/angular/file';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';
import { Pencil } from '@primeicons/angular/pencil';

@Component({
    template: `
        <p-treetable [value]="files" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                    <th style="width: 8rem">Actions</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            @if (editingKeys[rowData.key]) {
                                <input pInputText type="text" [(ngModel)]="rowData.name" pSize="small" class="flex-1" />
                            } @else {
                                <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                            }
                        </div>
                    </td>
                    <td>
                        @if (editingKeys[rowData.key]) {
                            <input pInputText type="text" [(ngModel)]="rowData.size" pSize="small" class="w-full" />
                        } @else {
                            {{ rowData.size }}
                        }
                    </td>
                    <td>
                        @if (editingKeys[rowData.key]) {
                            <input pInputText type="text" [(ngModel)]="rowData.type" pSize="small" class="w-full" />
                        } @else {
                            <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                        }
                    </td>
                    <td>
                        <div class="flex gap-1">
                            @if (editingKeys[rowData.key]) {
                                <button pButton text severity="success" size="small" (click)="onSave(rowData)">
                                    <svg data-p-icon="check" />
                                </button>
                                <button pButton text severity="danger" size="small" (click)="onCancel(rowData)">
                                    <svg data-p-icon="times" />
                                </button>
                            } @else {
                                <button pButton text severity="secondary" size="small" (click)="onInit(rowData)">
                                    <svg data-p-icon="pencil" />
                                </button>
                            }
                        </div>
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, ButtonModule, InputTextModule, FormsModule, Folder, Image, File, Check, Times, Pencil],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableRowEditDemo implements OnInit {
    files!: TreeNode[];

    editingKeys: { [key: string]: boolean } = {};

    private drafts: { [key: string]: any } = {};

    ngOnInit() {
        this.files = [
            {
                key: '0',
                expanded: true,
                data: { key: '0', name: 'Documents', size: '75kb', type: 'Folder' },
                children: [
                    {
                        key: '0-0',
                        data: { key: '0-0', name: 'Work', size: '55kb', type: 'Folder' },
                        children: [
                            { key: '0-0-0', data: { key: '0-0-0', name: 'Expenses.doc', size: '30kb', type: 'Document' } },
                            { key: '0-0-1', data: { key: '0-0-1', name: 'Resume.doc', size: '25kb', type: 'Document' } }
                        ]
                    },
                    {
                        key: '0-1',
                        data: { key: '0-1', name: 'Home', size: '20kb', type: 'Folder' },
                        children: [{ key: '0-1-0', data: { key: '0-1-0', name: 'Invoices.txt', size: '20kb', type: 'Text' } }]
                    }
                ]
            },
            {
                key: '1',
                data: { key: '1', name: 'Pictures', size: '150kb', type: 'Folder' },
                children: [
                    { key: '1-0', data: { key: '1-0', name: 'barcelona.jpg', size: '90kb', type: 'Picture' } },
                    { key: '1-1', data: { key: '1-1', name: 'primeui.png', size: '30kb', type: 'Picture' } }
                ]
            }
        ];
    }

    onInit(rowData: any) {
        this.drafts[rowData.key] = { ...rowData };
        this.editingKeys[rowData.key] = true;
    }

    onSave(rowData: any) {
        delete this.drafts[rowData.key];
        delete this.editingKeys[rowData.key];
    }

    onCancel(rowData: any) {
        const draft = this.drafts[rowData.key];

        if (draft) {
            rowData.name = draft.name;
            rowData.size = draft.size;
            rowData.type = draft.type;
        }

        delete this.drafts[rowData.key];
        delete this.editingKeys[rowData.key];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## scrollfrozencolumns-doc

A column can be fixed during horizontal scrolling by enabling the frozenColumns property.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="scrollableCols" [frozenColumns]="frozenLeftCols" [scrollable]="true" scrollHeight="400px" frozenWidth="300px">
            <ng-template #frozencolgroup>
                <colgroup>
                    <col style="width: 300px" />
                </colgroup>
            </ng-template>
            <ng-template #colgroup let-columns>
                <colgroup>
                    @for (col of columns; track col) {
                        <col style="width: 200px" />
                    }
                </colgroup>
            </ng-template>
            <ng-template #frozenheader>
                <tr>
                    <th>Name</th>
                </tr>
            </ng-template>
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th>{{ col.header }}</th>
                    }
                </tr>
            </ng-template>
            <ng-template #frozenbody let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode" style="height: 44px">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-semibold' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode" style="height: 44px">
                    @for (col of columns; let last = $last; track col) {
                        <td>
                            @if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else if (col.field === 'actions') {
                                <span class="text-xs text-surface-500 dark:text-surface-400">Open · Share</span>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableScrollFrozenColumnsDemo implements OnInit {
    files!: TreeNode[];

    frozenLeftCols!: Column[];

    scrollableCols!: Column[];

    ngOnInit() {
        const owners = ['Amy Elsner', 'Anna Fali', 'Bernardo Dominic', 'Ioni Bowcher', 'Stephen Shaw'];
        const types = ['Document', 'Text', 'Picture', 'Video'];
        this.files = Array.from({ length: 10 }, (_, i) => ({
            key: String(i),
            expanded: i < 2,
            data: {
                name: `Folder ${i + 1}`,
                size: '—',
                type: 'Folder',
                owner: owners[i % owners.length],
                modified: `2026-0${(i % 9) + 1}-15`
            },
            children: Array.from({ length: 5 }, (_, j) => {
                const type = types[j % types.length];
                const size = ((i * 37 + j * 113) % 1950) + 50;
                return {
                    key: `${i}-${j}`,
                    data: {
                        name: `${type} ${i + 1}.${j + 1}`,
                        size: `${size}kb`,
                        type,
                        owner: owners[(i + j) % owners.length],
                        modified: `2026-0${((i + j) % 9) + 1}-20`
                    }
                };
            })
        }));
        this.frozenLeftCols = [{ field: 'name', header: 'Name' }];
        this.scrollableCols = [
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' },
            { field: 'owner', header: 'Owner' },
            { field: 'modified', header: 'Modified' },
            { field: 'actions', header: 'Actions' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## scrollhorizontal-doc

Horizontal scrolling is enabled when the total width of columns exceeds table width.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" scrollHeight="250px" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #colgroup let-columns>
                <colgroup>
                    @for (col of columns; let first = $first; track col) {
                        <col [style.width]="first ? '350px' : '250px'" />
                    }
                </colgroup>
            </ng-template>
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col) {
                        <th>{{ col.header }}</th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableScrollHorizontalDemo implements OnInit {
    files!: TreeNode[];

    cols!: Column[];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = this.enrichNodes(files);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' },
            { field: 'key', header: 'Key' },
            { field: 'level', header: 'Level' },
            { field: 'hasChildren', header: 'Has Children' }
        ];
    }

    enrichNodes(nodes: TreeNode[], level = 0): TreeNode[] {
        return nodes.map((node) => ({
            ...node,
            data: { ...node.data, key: node.key, level, hasChildren: node.children?.length ? 'Yes' : 'No' },
            children: node.children ? this.enrichNodes(node.children, level + 1) : undefined
        }));
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## scrollvertical-doc

Vertical scrolling with fixed headers is enabled by setting scrollable and scrollHeight properties.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [scrollable]="true" scrollHeight="400px" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableScrollVerticalDemo implements OnInit {
    files!: TreeNode[];

    ngOnInit() {
        const types = ['Document', 'Text', 'Picture', 'Video'];
        this.files = Array.from({ length: 20 }, (_, i) => ({
            key: String(i),
            expanded: i < 2,
            data: { name: `Folder ${i + 1}`, size: '—', type: 'Folder' },
            children: Array.from({ length: 6 }, (_, j) => {
                const type = types[j % types.length];
                const size = ((i * 37 + j * 113) % 1950) + 50;
                return {
                    key: `${i}-${j}`,
                    data: {
                        name: `${type} ${i + 1}.${j + 1}`,
                        size: `${size}kb`,
                        type
                    }
                };
            })
        }));
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## selectioncheckbox-doc

Checkbox column with propagating selection — toggling a parent selects all descendants, partial state renders as indeterminate.

```typescript
import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" selectionMode="checkbox" [(selectionKeys)]="selectionKeys" (selectionKeysChange)="onSelectionChange()" dataKey="key" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>
                        <div class="flex items-center gap-2">
                            <p-treetable-header-checkbox class="flex" />
                            <span>Name</span>
                        </div>
                    </th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode" [ttSelectableRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            <p-treetable-checkbox [value]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSelectionCheckboxDemo implements OnInit {
    files!: TreeNode[];

    selectionKeys: { [key: string]: any } = {};

    selectedLabel: string = 'None';

    ngOnInit() {
        this.files = [
            {
                key: '0',
                expanded: true,
                data: { name: 'Documents', size: '75kb', type: 'Folder' },
                children: [
                    {
                        key: '0-0',
                        expanded: true,
                        data: { name: 'Work', size: '55kb', type: 'Folder' },
                        children: [
                            { key: '0-0-0', data: { name: 'Expenses.doc', size: '30kb', type: 'Document' } },
                            { key: '0-0-1', data: { name: 'Resume.doc', size: '25kb', type: 'Document' } }
                        ]
                    },
                    {
                        key: '0-1',
                        data: { name: 'Home', size: '20kb', type: 'Folder' },
                        children: [{ key: '0-1-0', data: { name: 'Invoices.txt', size: '20kb', type: 'Text' } }]
                    }
                ]
            },
            {
                key: '1',
                data: { name: 'Pictures', size: '150kb', type: 'Folder' },
                children: [
                    { key: '1-0', data: { name: 'barcelona.jpg', size: '90kb', type: 'Picture' } },
                    { key: '1-1', data: { name: 'primeui.png', size: '30kb', type: 'Picture' } },
                    { key: '1-2', data: { name: 'optimus.jpg', size: '30kb', type: 'Picture' } }
                ]
            },
            {
                key: '2',
                data: { name: 'Movies', size: '1500kb', type: 'Folder' },
                children: [
                    {
                        key: '2-0',
                        data: { name: 'Al Pacino', size: '1000kb', type: 'Folder' },
                        children: [
                            { key: '2-0-0', data: { name: 'Scarface', size: '500kb', type: 'Video' } },
                            { key: '2-0-1', data: { name: 'Serpico', size: '500kb', type: 'Video' } }
                        ]
                    },
                    {
                        key: '2-1',
                        data: { name: 'Robert De Niro', size: '500kb', type: 'Folder' },
                        children: [
                            { key: '2-1-0', data: { name: 'Goodfellas', size: '250kb', type: 'Video' } },
                            { key: '2-1-1', data: { name: 'Untouchables', size: '250kb', type: 'Video' } }
                        ]
                    }
                ]
            }
        ];
    }

    onSelectionChange() {
        const keys = Object.keys(this.selectionKeys).filter((k) => this.selectionKeys[k]?.checked);

        this.selectedLabel = keys.length ? keys.join(', ') : 'None';
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## selectionevents-doc

TreeTable provides onNodeSelect and onNodeUnselect events to listen selection events.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode, MessageService } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface NodeEvent {
    originalEvent?: Event;
    node?: TreeNode;
}

@Component({
    template: `
        <p-treetable [value]="files" selectionMode="single" [(selection)]="selectedNode" dataKey="name" (onNodeSelect)="nodeSelect($event)" (onNodeUnselect)="nodeUnselect($event)" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode" [ttSelectableRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService, MessageService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSelectionEventsDemo implements OnInit {
    files!: TreeNode[];

    selectedNode!: TreeNode;

    private nodeService = inject(NodeService);

    private messageService = inject(MessageService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    nodeSelect(event: NodeEvent) {
        this.messageService.add({ severity: 'info', summary: 'Node Selected', detail: event.node.data.name });
    }

    nodeUnselect(event: NodeEvent) {
        this.messageService.add({ severity: 'warn', summary: 'Node Unselected', detail: event.node.data.name });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Multiple

More than one node is selectable by setting selectionMode to multiple . By default in multiple selection mode, metaKey press (e.g. ⌘ ) is necessary to add to existing selections however this can be configured with disabling the metaKeySelection property. Note that in touch enabled devices, TreeTable always ignores metaKey.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTable, TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" selectionMode="multiple" [(selection)]="selectedNodes" dataKey="name" [metaKeySelection]="true" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode" [ttSelectableRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSelectionMultipleDemo implements OnInit {
    files!: TreeNode[];

    selectedNodes!: TreeNode[];

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Single

Single node selection is configured by setting selectionMode as single .

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" selectionMode="single" [(selection)]="selectedNode" dataKey="name" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode" [ttSelectableRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSelectionSingleDemo implements OnInit {
    files!: TreeNode[];

    selectedNode!: TreeNode;

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Size

Use the size property with small or large to adjust cell padding.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-treetable [value]="files" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }" [class]="selectedSize">
            <ng-template #header>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Type</th>
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData">
                <tr [ttRow]="rowNode">
                    <td>
                        <div class="flex items-center gap-2">
                            <p-treetable-toggler [rowNode]="rowNode" />
                            @switch (rowData.type) {
                                @case ('Folder') {
                                    <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Picture') {
                                    <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @case ('Video') {
                                    <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                                @default {
                                    <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                }
                            }
                            <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData.name }}</span>
                        </div>
                    </td>
                    <td>
                        <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData.size }}</span>
                    </td>
                    <td>
                        <p-tag [value]="rowData.type" [severity]="getSeverity(rowData.type)" />
                    </td>
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSizeDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    sizes!: any[];

    selectedSize: any = '';

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.sizes = [
            { name: 'Small', class: 'p-treetable-sm' },
            { name: 'Normal', class: '' },
            { name: 'Large', class: 'p-treetable-lg' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Multiple Columns

Multiple columns can be sorted by defining sortMode as multiple . This mode requires metaKey (e.g. ⌘ ) to add to existing sort columns.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" sortMode="multiple" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; let first = $first; track col) {
                        <th [ttSortableColumn]="col.field">
                            <div class="flex items-center gap-2">
                                {{ col.header }}
                                <p-treetable-sort-icon [field]="col.field" />
                            </div>
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSortMultipleColumnsDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## sortremovable-doc

The third consecutive click on a sorted column removes the sorting and restores the original order.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TreeTable, TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable
            #tt
            [value]="files"
            (onSort)="onSort($event)"
            [columns]="cols"
            selectionMode="single"
            [metaKeySelection]="metaKeySelection"
            [(selection)]="selectedNode"
            dataKey="name"
            [scrollable]="true"
            [tableStyle]="{ 'min-width': '50rem' }"
        >
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col.field) {
                        <th [ttSortableColumn]="col.field">
                            {{ col.header }}
                            <p-treetable-sort-icon [field]="col.field" />
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode" [ttSelectableRow]="rowNode">
                    @for (col of columns; track col.field; let i = $index) {
                        <td>
                            @if (i === 0) {
                                <div class="inline-flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    <span>{{ rowData[col.field] }}</span>
                                </div>
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSortRemovableDemo implements OnInit {
    private nodeService = inject(NodeService);

    metaKeySelection: boolean = true;

    files!: TreeNode[];

    initialValue: TreeNode[];

    selectedNode!: TreeNode;

    cols!: Column[];

    lastSort: SortMeta | null = null;

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => {
            this.files = files;
            this.initialValue = structuredClone(files);
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    onSort(event: SortMeta) {
        if (this.lastSort && this.lastSort.field === event.field && this.lastSort.order === -1 && event.order === 1) {
            this.tt._sortField = null;
            this.tt._sortOrder = this.tt.defaultSortOrder();
            this.tt.tableService.onSort(null);

            this.files = structuredClone(this.initialValue);
            this.lastSort = null;
        } else {
            this.lastSort = { field: event.field, order: event.order };
        }
    }
}
```

## Single Column

Sorting is evaluated per tree level — siblings are sorted against each other while the hierarchy stays intact.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TagModule } from 'primeng/tag';
import { TreeTableModule } from 'primeng/treetable';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Image } from '@primeicons/angular/image';
import { Video } from '@primeicons/angular/video';
import { File } from '@primeicons/angular/file';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [scrollable]="true" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; let first = $first; track col) {
                        <th [ttSortableColumn]="col.field" [style.width]="first ? '50%' : ''">
                            <div class="flex items-center gap-2">
                                {{ col.header }}
                                <p-treetable-sort-icon [field]="col.field" />
                            </div>
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; track col) {
                        <td [style.width]="first ? '50%' : ''">
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    @switch (rowData.type) {
                                        @case ('Folder') {
                                            <svg data-p-icon="folder" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Picture') {
                                            <svg data-p-icon="image" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @case ('Video') {
                                            <svg data-p-icon="video" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                        @default {
                                            <svg data-p-icon="file" [size]="16" class="text-surface-500 dark:text-surface-400" />
                                        }
                                    }
                                    <span [class]="rowData.type === 'Folder' ? 'font-medium' : ''">{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (col.field === 'size') {
                                <span class="text-sm text-surface-500 dark:text-surface-400">{{ rowData[col.field] }}</span>
                            } @else if (col.field === 'type') {
                                <p-tag [value]="rowData[col.field]" [severity]="getSeverity(rowData[col.field])" />
                            } @else {
                                {{ rowData[col.field] }}
                            }
                        </td>
                    }
                </tr>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TagModule, TreeTableModule, Folder, Image, Video, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableSortSingleColumnDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getTreeTableNodes().then((files) => {
            this.files = files.slice(0, 5);
            this.files[0].expanded = true;
        });
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' }
        ];
    }

    getSeverity(type: string): 'info' | 'warn' | 'success' | 'secondary' {
        switch (type) {
            case 'Folder':
                return 'warn';
            case 'Document':
                return 'info';
            case 'Picture':
            case 'Video':
                return 'success';
            default:
                return 'secondary';
        }
    }
}
```

## Template

Custom content at caption , header , body and summary sections are supported via templating.

```typescript
import { Component, OnInit, inject, ChangeDetectionStrategy } from '@angular/core';
import { TreeTableModule } from 'primeng/treetable';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

interface Column {
    field: string;
    header: string;
}

@Component({
    template: `
        <p-treetable [value]="files" [columns]="cols" [tableStyle]="{ 'min-width': '50rem' }">
            <ng-template #caption><div class="text-xl font-bold">File Viewer</div> </ng-template>
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; let last = $last; track col) {
                        <th [class]="{ 'w-40': last }">
                            {{ col.header }}
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowNode let-rowData="rowData" let-columns="columns">
                <tr [ttRow]="rowNode">
                    @for (col of columns; let first = $first; let last = $last; track col) {
                        <td>
                            @if (first) {
                                <div class="flex items-center gap-2">
                                    <p-treetable-toggler [rowNode]="rowNode" />
                                    <span>{{ rowData[col.field] }}</span>
                                </div>
                            } @else if (last) {
                                <div class="flex flex-wrap gap-2">
                                    <button pButton iconOnly rounded="true" severity="secondary"><i class="pi pi-search"></i></button>
                                    <button pButton iconOnly rounded="true" severity="secondary"><i class="pi pi-pencil"></i></button>
                                </div>
                            } @else {
                                <span>{{ rowData[col.field] }}</span>
                            }
                        </td>
                    }
                </tr>
            </ng-template>
            <ng-template #summary>
                <div style="text-align:left">
                    <button pButton severity="warn"><i class="pi pi-refresh"></i> Reload</button>
                </div>
            </ng-template>
        </p-treetable>
    `,
    standalone: true,
    imports: [TreeTableModule, ButtonModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeTableTemplateDemo implements OnInit {
    private nodeService = inject(NodeService);

    files!: TreeNode[];

    cols!: Column[];

    ngOnInit() {
        this.nodeService.getFilesystem().then((files) => (this.files = files));
        this.cols = [
            { field: 'name', header: 'Name' },
            { field: 'size', header: 'Size' },
            { field: 'type', header: 'Type' },
            { field: '', header: '' }
        ];
    }
}
```

## Tree Table

TreeTable is used to display hierarchical data in tabular format.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, TreeTablePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| columns | any[] \| undefined | - | An array of objects to represent dynamic columns. |
| tableStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the table. |
| tableStyleClass | string \| undefined | - | Style class of the table. |
| autoLayout | boolean \| undefined | - | Whether the cell widths scale according to their content or not. |
| lazy | boolean | - | Defines if data is loaded and interacted with in lazy manner. |
| lazyLoadOnInit | boolean | - | Whether to call lazy loading on initialization. |
| paginator | boolean \| undefined | - | When specified as true, enables the pagination. |
| rows | number \| undefined | - | Number of rows to display per page. |
| firstInput | number | - | Index of the first row to be displayed. |
| pageLinks | number | - | Number of page links to display in paginator. |
| rowsPerPageOptions | any[] \| undefined | - | Array of integer/object values to display inside rows per page dropdown of paginator |
| alwaysShowPaginator | boolean | - | Whether to show it even there is only one page. |
| paginatorPosition | "top" \| "bottom" \| "both" | - | Position of the paginator. |
| paginatorStyleClass | string \| undefined | - | Custom style class for paginator |
| paginatorDropdownAppendTo | any | - | Target element to attach the paginator dropdown overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| currentPageReportTemplate | string | - | Template of the current page report element. Available placeholders are {currentPage},{totalPages},{rows},{first},{last} and {totalRecords} |
| showCurrentPageReport | boolean \| undefined | - | Whether to display current page report. |
| showJumpToPageDropdown | boolean \| undefined | - | Whether to display a dropdown to navigate to any page. |
| showFirstLastIcon | boolean | - | When enabled, icons are displayed on paginator to go first and last page. |
| showPageLinks | boolean | - | Whether to show page links. |
| defaultSortOrder | number | - | Sort order to use when an unsorted column gets sorted by user interaction. |
| sortMode | "single" \| "multiple" | - | Defines whether sorting works on single column or on multiple columns. |
| resetPageOnSort | boolean | - | When true, resets paginator to first page after sorting. |
| customSort | boolean \| undefined | - | Whether to use the default sorting or a custom one using sortFunction. |
| selectionMode | TreeTableSelectionMode \| undefined | - | Specifies the selection mode, valid values are "single" and "multiple". |
| contextMenuSelection | any | - | Selected row with a context menu. |
| dataKey | string \| undefined | - | A property to uniquely identify a record in data. |
| metaKeySelection | boolean | - | Defines whether metaKey is should be considered for the selection. On touch enabled devices, metaKeySelection is turned off automatically. |
| compareSelectionBy | "equals" \| "deepEquals" | - | Algorithm to define if a row is selected, valid values are "equals" that compares by reference and "deepEquals" that compares all fields. |
| rowHover | boolean \| undefined | - | Adds hover effect to rows without the need for selectionMode. |
| loading | boolean \| undefined | - | Displays a loader to indicate data load is in progress. |
| loadingIcon | string \| undefined | - | The icon to show while indicating data load is in progress. |
| showLoader | boolean | - | Whether to show the loading mask when loading property is true. |
| scrollable | boolean \| undefined | - | When specified, enables horizontal and/or vertical scrolling. |
| scrollHeight | string \| undefined | - | Height of the scroll viewport in fixed pixels or the "flex" keyword for a dynamic size. |
| virtualScroll | boolean \| undefined | - | Whether the data should be loaded on demand during scroll. |
| virtualScrollItemSize | number \| undefined | - | Height of a row to use in calculations of virtual scrolling. |
| virtualScrollOptions | ScrollerOptions \| undefined | - | Whether to use the scroller feature. The properties of scroller component can be used like an object in it. |
| virtualScrollDelay | number | - | The delay (in milliseconds) before triggering the virtual scroll. This determines the time gap between the user's scroll action and the actual rendering of the next set of items in the virtual scroll. |
| frozenWidth | string \| undefined | - | Width of the frozen columns container. |
| frozenColumns | any | - | An array of objects to represent dynamic columns that are frozen. |
| resizableColumns | boolean \| undefined | - | When enabled, columns can be resized using drag and drop. |
| columnResizeMode | "fit" \| "expand" | - | Defines whether the overall table width should change on column resize, valid values are "fit" and "expand". |
| reorderableColumns | boolean \| undefined | - | When enabled, columns can be reordered using drag and drop. |
| contextMenu | any | - | Local ng-template varilable of a ContextMenu. |
| rowTrackBy | Function | - | Function to optimize the dom operations by delegating to ngForTrackBy, default algorithm checks for object identity. |
| filtersInput | { [s: string]: FilterMetadata \| undefined } | - | An array of FilterMetadata objects to provide external filters. |
| globalFilterFields | string[] \| undefined | - | An array of fields as string to use in global filtering. |
| filterDelay | number | - | Delay in milliseconds before filtering the data. |
| filterMode | "lenient" \| "strict" | - | Mode for filtering valid values are "lenient" and "strict". Default is lenient. |
| filterLocale | string \| undefined | - | Locale to use in filtering. The default locale is the host environment's current locale. |
| paginatorLocale | string \| undefined | - | Locale to be used in paginator formatting. |
| totalRecordsInput | number | - | Number of total records, defaults to length of value when not defined. |
| sortFieldInput | string \| null \| undefined | - | Name of the field to sort data by default. |
| sortOrderInput | number | 1 | Order to sort when default sorting is enabled. |
| multiSortMetaInput | SortMeta[] \| null \| undefined | null | An array of SortMeta objects to sort the data by default in multiple sort mode. |
| selectionInput | any | null | Selected row in single mode or an array of values in multiple mode. **(Deprecated)** |
| valueInput | TreeNode<any>[] \| undefined | null | An array of objects to display. |
| selectionKeys | any | - | A map of keys to control the selection state. |
| collapsedKeys | any | - | A map of keys (by  `dataKey`  value) whose value is  `true`  for collapsed rows. Two-way bindable. When provided it controls row expansion (a row is expanded unless its key maps to  `true` ); leave it unset to keep driving expansion from each node's  `expanded`  flag (default, non-breaking). |
| showGridlines | boolean | false | Whether to show grid lines between cells. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| selectionChange | value: TreeTableNode<any | Callback to invoke on selected node change. |
| contextMenuSelectionChange | value: TreeTableNode<any | Callback to invoke on context menu selection change. |
| onFilter | event: TreeTableFilterEvent | Callback to invoke when data is filtered. |
| onNodeExpand | event: TreeTableNodeExpandEvent<any | Callback to invoke when a node is expanded. |
| onNodeCollapse | event: TreeTableNodeCollapseEvent<any | Callback to invoke when a node is collapsed. |
| onPage | value: TreeTablePaginatorState | Callback to invoke when pagination occurs. |
| onSort | value: any | Callback to invoke when a column gets sorted. |
| onLazyLoad | event: TreeTableLazyLoadEvent | Callback to invoke when paging, sorting or filtering happens in lazy mode. |
| sortFunction | event: TreeTableSortEvent | An event emitter to invoke on custom sorting, refer to sorting section for details. |
| onColResize | event: TreeTableColResizeEvent | Callback to invoke when a column is resized. |
| onColReorder | event: TreeTableColumnReorderEvent | Callback to invoke when a column is reordered. |
| onNodeSelect | value: TreeTableNode<any | Callback to invoke when a node is selected. |
| onNodeUnselect | event: TreeTableNodeUnSelectEvent | Callback to invoke when a node is unselected. |
| onContextMenuSelect | event: TreeTableContextMenuSelectEvent | Callback to invoke when a node is selected with right click. |
| onHeaderCheckboxToggle | event: TreeTableHeaderCheckboxToggleEvent | Callback to invoke when state of header checkbox changes. |
| onEditInit | event: TreeTableEditEvent | Callback to invoke when a cell switches to edit mode. |
| onEditComplete | event: TreeTableEditEvent | Callback to invoke when cell edit is completed. |
| onEditCancel | event: TreeTableEditEvent | Callback to invoke when cell edit is cancelled with escape key. |
| selectionKeysChange | value: any | Callback to invoke when selectionKeys are changed. |
| collapsedKeysChange | value: any | Callback to invoke when collapsedKeys are changed. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| colgroup | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| caption | TemplateRef<void> \| undefined |  |
| header | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| body | TemplateRef<TreeTableBodyTemplateContext> \| undefined |  |
| footer | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| summary | TemplateRef<void> \| undefined |  |
| emptymessage | TemplateRef<TreeTableEmptyMessageTemplateContext> \| undefined |  |
| paginatorleft | TemplateRef<PaginatorTemplateContext> \| undefined |  |
| paginatorright | TemplateRef<PaginatorTemplateContext> \| undefined |  |
| paginatordropdownitem | TemplateRef<PaginatorDropdownItemTemplateContext> \| undefined |  |
| frozenheader | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| frozenbody | TemplateRef<void> \| undefined |  |
| frozenfooter | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| frozencolgroup | TemplateRef<TreeTableColumnsTemplateContext> \| undefined |  |
| loadingicon | TemplateRef<void> \| undefined |  |
| reorderindicatorupicon | TemplateRef<void> \| undefined |  |
| reorderindicatordownicon | TemplateRef<void> \| undefined |  |
| sorticon | TemplateRef<TreeTableSortIconTemplateContext> \| undefined |  |
| checkboxicon | TemplateRef<TreeTableCheckboxIconTemplateContext> \| undefined |  |
| headercheckboxicon | TemplateRef<TreeTableHeaderCheckboxIconTemplateContext> \| undefined |  |
| togglericon | TemplateRef<TreeTableTogglerIconTemplateContext> \| undefined |  |
| paginatorfirstpagelinkicon | TemplateRef<void> \| undefined |  |
| paginatorlastpagelinkicon | TemplateRef<void> \| undefined |  |
| paginatorpreviouspagelinkicon | TemplateRef<void> \| undefined |  |
| paginatornextpagelinkicon | TemplateRef<void> \| undefined |  |
| loader | TemplateRef<ScrollerLoaderTemplateContext> \| undefined |  |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| resetScrollTop |  | void | Resets scroll to top. |
| scrollToVirtualIndex | index: number | void | Scrolls to given index when using virtual scroll. |
| scrollTo | options: ScrollToOptions | void | Scrolls to given index. |
| reset |  | void | Clears the sort and paginator state. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| loading | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the loading's DOM element. |
| mask | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the mask's DOM element. |
| loadingIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the loading icon's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| pcPaginator | PaginatorPassThrough | Used to pass attributes to the Paginator component. |
| wrapper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the wrapper's DOM element. |
| table | PassThroughOption<HTMLTableElement, I> | Used to pass attributes to the table's DOM element. |
| thead | PassThroughOption<HTMLTableSectionElement, I> | Used to pass attributes to the thead's DOM element. |
| tbody | PassThroughOption<HTMLTableSectionElement, I> | Used to pass attributes to the tbody's DOM element. |
| tfoot | PassThroughOption<HTMLTableSectionElement, I> | Used to pass attributes to the tfoot's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |
| scrollableWrapper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable wrapper's DOM element. |
| scrollableView | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable container's DOM element. |
| scrollableHeader | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable header's DOM element. |
| scrollableHeaderBox | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable header box's DOM element. |
| scrollableHeaderTable | PassThroughOption<HTMLTableElement, I> | Used to pass attributes to the scrollable header table's DOM element. |
| virtualScroller | VirtualScrollerPassThrough | Used to pass attributes to the Scroller component. |
| scrollableBody | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable body's DOM element. |
| scrollableFooter | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable footer's DOM element. |
| scrollableFooterBox | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollable footer box's DOM element. |
| scrollableFooterTable | PassThroughOption<HTMLTableElement, I> | Used to pass attributes to the scrollable footer table's DOM element. |
| columnResizerHelper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the column resizer helper's DOM element. |
| reorderIndicatorUp | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the reorder indicator up's DOM element. |
| reorderIndicatorDown | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the reorder indicator down's DOM element. |
| sortableColumn | PassThroughOption<HTMLTableCellElement, I> | Used to pass attributes to the sortable column's DOM element. |
| sortableColumnIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the sortable column icon's DOM element. |
| pcSortableColumnBadge | BadgePassThrough | Used to pass attributes to the Badge component for sortable column. |
| row | PassThroughOption<HTMLTableRowElement, I> | Used to pass attributes to the row's DOM element. |
| pcRowCheckbox | CheckboxPassThrough | Used to pass attributes to the Checkbox component for row. |
| pcHeaderCheckbox | CheckboxPassThrough | Used to pass attributes to the Checkbox component for header. |
| cellEditor | PassThroughOption<HTMLElement, I> | Used to pass attributes to the cell editor's DOM element. |
| rowToggleButton | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the row toggle button's DOM element. |
| toggler | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the toggler's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-treetable | Class name of the root element |
| p-treetable-loading | Class name of the loading element |
| p-treetable-mask | Class name of the mask element |
| p-treetable-loading-icon | Class name of the loading icon element |
| p-treetable-header | Class name of the header element |
| p-treetable-paginator-[position] | Class name of the paginator element |
| p-treetable-table-container | Class name of the table container element |
| p-treetable-table | Class name of the table element |
| p-treetable-thead | Class name of the thead element |
| p-treetable-column-resizer | Class name of the column resizer element |
| p-treetable-column-title | Class name of the column title element |
| p-treetable-sort-icon | Class name of the sort icon element |
| p-treetable-sort-badge | Class name of the sort badge element |
| p-treetable-tbody | Class name of the tbody element |
| p-treetable-node-toggle-button | Class name of the node toggle button element |
| p-treetable-node-toggle-icon | Class name of the node toggle icon element |
| p-treetable-node-checkbox | Class name of the node checkbox element |
| p-treetable-empty-message | Class name of the empty message element |
| p-treetable-tfoot | Class name of the tfoot element |
| p-treetable-footer | Class name of the footer element |
| p-treetable-column-resize-indicator | Class name of the column resize indicator element |
| p-treetable-wrapper | Class name of the wrapper element |
| p-treetable-scrollable-wrapper | Class name of the scrollable wrapper element |
| p-treetable-scrollable-view | Class name of the scrollable view element |
| p-treetable-frozen-view | Class name of the frozen view element |
| p-treetable-column-resizer-helper | Class name of the column resizer helper element |
| p-treetable-reorder-indicator-up | Class name of the reorder indicator up element |
| p-treetable-reorder-indicator-down | Class name of the reorder indicator down element |
| p-treetable-scrollable-header | Class name of the scrollable header element |
| p-treetable-scrollable-header-box | Class name of the scrollable header box element |
| p-treetable-scrollable-header-table | Class name of the scrollable header table element |
| p-treetable-scrollable-body | Class name of the scrollable body element |
| p-treetable-scrollable-footer | Class name of the scrollable footer element |
| p-treetable-scrollable-footer-box | Class name of the scrollable footer box element |
| p-treetable-scrollable-footer-table | Class name of the scrollable footer table element |
| p-sortable-column-icon | Class name of the sortable column icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| treetable.transition.duration | --p-treetable-transition-duration | Transition duration of root |
| treetable.border.color | --p-treetable-border-color | Border color of root |
| treetable.header.background | --p-treetable-header-background | Background of header |
| treetable.header.border.color | --p-treetable-header-border-color | Border color of header |
| treetable.header.color | --p-treetable-header-color | Color of header |
| treetable.header.border.width | --p-treetable-header-border-width | Border width of header |
| treetable.header.padding | --p-treetable-header-padding | Padding of header |
| treetable.header.cell.background | --p-treetable-header-cell-background | Background of header cell |
| treetable.header.cell.hover.background | --p-treetable-header-cell-hover-background | Hover background of header cell |
| treetable.header.cell.selected.background | --p-treetable-header-cell-selected-background | Selected background of header cell |
| treetable.header.cell.border.color | --p-treetable-header-cell-border-color | Border color of header cell |
| treetable.header.cell.color | --p-treetable-header-cell-color | Color of header cell |
| treetable.header.cell.hover.color | --p-treetable-header-cell-hover-color | Hover color of header cell |
| treetable.header.cell.selected.color | --p-treetable-header-cell-selected-color | Selected color of header cell |
| treetable.header.cell.gap | --p-treetable-header-cell-gap | Gap of header cell |
| treetable.header.cell.padding | --p-treetable-header-cell-padding | Padding of header cell |
| treetable.header.cell.focus.ring.width | --p-treetable-header-cell-focus-ring-width | Focus ring width of header cell |
| treetable.header.cell.focus.ring.style | --p-treetable-header-cell-focus-ring-style | Focus ring style of header cell |
| treetable.header.cell.focus.ring.color | --p-treetable-header-cell-focus-ring-color | Focus ring color of header cell |
| treetable.header.cell.focus.ring.offset | --p-treetable-header-cell-focus-ring-offset | Focus ring offset of header cell |
| treetable.header.cell.focus.ring.shadow | --p-treetable-header-cell-focus-ring-shadow | Focus ring shadow of header cell |
| treetable.column.title.font.weight | --p-treetable-column-title-font-weight | Font weight of column title |
| treetable.column.title.font.size | --p-treetable-column-title-font-size | Font size of column title |
| treetable.row.background | --p-treetable-row-background | Background of row |
| treetable.row.hover.background | --p-treetable-row-hover-background | Hover background of row |
| treetable.row.selected.background | --p-treetable-row-selected-background | Selected background of row |
| treetable.row.color | --p-treetable-row-color | Color of row |
| treetable.row.hover.color | --p-treetable-row-hover-color | Hover color of row |
| treetable.row.selected.color | --p-treetable-row-selected-color | Selected color of row |
| treetable.row.focus.ring.width | --p-treetable-row-focus-ring-width | Focus ring width of row |
| treetable.row.focus.ring.style | --p-treetable-row-focus-ring-style | Focus ring style of row |
| treetable.row.focus.ring.color | --p-treetable-row-focus-ring-color | Focus ring color of row |
| treetable.row.focus.ring.offset | --p-treetable-row-focus-ring-offset | Focus ring offset of row |
| treetable.row.focus.ring.shadow | --p-treetable-row-focus-ring-shadow | Focus ring shadow of row |
| treetable.body.cell.border.color | --p-treetable-body-cell-border-color | Border color of body cell |
| treetable.body.cell.padding | --p-treetable-body-cell-padding | Padding of body cell |
| treetable.body.cell.gap | --p-treetable-body-cell-gap | Gap of body cell |
| treetable.body.cell.selected.border.color | --p-treetable-body-cell-selected-border-color | Selected border color of body cell |
| treetable.body.cell.font.weight | --p-treetable-body-cell-font-weight | Font weight of body cell |
| treetable.body.cell.font.size | --p-treetable-body-cell-font-size | Font size of body cell |
| treetable.footer.cell.background | --p-treetable-footer-cell-background | Background of footer cell |
| treetable.footer.cell.border.color | --p-treetable-footer-cell-border-color | Border color of footer cell |
| treetable.footer.cell.color | --p-treetable-footer-cell-color | Color of footer cell |
| treetable.footer.cell.padding | --p-treetable-footer-cell-padding | Padding of footer cell |
| treetable.column.footer.font.weight | --p-treetable-column-footer-font-weight | Font weight of column footer |
| treetable.column.footer.font.size | --p-treetable-column-footer-font-size | Font size of column footer |
| treetable.footer.background | --p-treetable-footer-background | Background of footer |
| treetable.footer.border.color | --p-treetable-footer-border-color | Border color of footer |
| treetable.footer.color | --p-treetable-footer-color | Color of footer |
| treetable.footer.border.width | --p-treetable-footer-border-width | Border width of footer |
| treetable.footer.padding | --p-treetable-footer-padding | Padding of footer |
| treetable.column.resizer.width | --p-treetable-column-resizer-width | Width of column resizer |
| treetable.resize.indicator.width | --p-treetable-resize-indicator-width | Width of resize indicator |
| treetable.resize.indicator.color | --p-treetable-resize-indicator-color | Color of resize indicator |
| treetable.sort.icon.color | --p-treetable-sort-icon-color | Color of sort icon |
| treetable.sort.icon.hover.color | --p-treetable-sort-icon-hover-color | Hover color of sort icon |
| treetable.sort.icon.size | --p-treetable-sort-icon-size | Size of sort icon |
| treetable.loading.icon.size | --p-treetable-loading-icon-size | Size of loading icon |
| treetable.node.toggle.button.hover.background | --p-treetable-node-toggle-button-hover-background | Hover background of node toggle button |
| treetable.node.toggle.button.selected.hover.background | --p-treetable-node-toggle-button-selected-hover-background | Selected hover background of node toggle button |
| treetable.node.toggle.button.color | --p-treetable-node-toggle-button-color | Color of node toggle button |
| treetable.node.toggle.button.hover.color | --p-treetable-node-toggle-button-hover-color | Hover color of node toggle button |
| treetable.node.toggle.button.selected.hover.color | --p-treetable-node-toggle-button-selected-hover-color | Selected hover color of node toggle button |
| treetable.node.toggle.button.size | --p-treetable-node-toggle-button-size | Size of node toggle button |
| treetable.node.toggle.button.border.radius | --p-treetable-node-toggle-button-border-radius | Border radius of node toggle button |
| treetable.node.toggle.button.focus.ring.width | --p-treetable-node-toggle-button-focus-ring-width | Focus ring width of node toggle button |
| treetable.node.toggle.button.focus.ring.style | --p-treetable-node-toggle-button-focus-ring-style | Focus ring style of node toggle button |
| treetable.node.toggle.button.focus.ring.color | --p-treetable-node-toggle-button-focus-ring-color | Focus ring color of node toggle button |
| treetable.node.toggle.button.focus.ring.offset | --p-treetable-node-toggle-button-focus-ring-offset | Focus ring offset of node toggle button |
| treetable.node.toggle.button.focus.ring.shadow | --p-treetable-node-toggle-button-focus-ring-shadow | Focus ring shadow of node toggle button |
| treetable.paginator.top.border.color | --p-treetable-paginator-top-border-color | Border color of paginator top |
| treetable.paginator.top.border.width | --p-treetable-paginator-top-border-width | Border width of paginator top |
| treetable.paginator.bottom.border.color | --p-treetable-paginator-bottom-border-color | Border color of paginator bottom |
| treetable.paginator.bottom.border.width | --p-treetable-paginator-bottom-border-width | Border width of paginator bottom |
