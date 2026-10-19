---
title: "Angular Tree Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/tree.md"
description: "Tree is used to display hierarchical data."
---

# Angular Tree Component

Tree is used to display hierarchical data.

## Accessibility

Screen Reader Value to describe the component can either be provided with aria-labelledby or aria-label props. The root list element has a tree role whereas each list item has a treeitem role along with aria-label , aria-selected and aria-expanded attributes. In checkbox selection, aria-checked is used instead of aria-selected . The container element of a treenode has the group role. Checkbox and toggle icons are hidden from screen readers as their parent element with treeitem role and attributes are used instead for readers and keyboard support. The aria-setsize , aria-posinset and aria-level attributes are calculated implicitly and added to each treeitem. Keyboard Support Key Function tab Moves focus to the first selected node when focus enters the component, if there is none then first element receives the focus. If focus is already inside the component, moves focus to the next focusable element in the page tab sequence. shift + tab Moves focus to the last selected node when focus enters the component, if there is none then first element receives the focus. If focus is already inside the component, moves focus to the previous focusable element in the page tab sequence. enter Selects the focused treenode. space Selects the focused treenode. down arrow Moves focus to the next treenode. up arrow Moves focus to the previous treenode. right arrow If node is closed, opens the node otherwise moves focus to the first child node. left arrow If node is open, closes the node otherwise moves focus to the parent node.

## Basic

Displays hierarchical data with expand and collapse support.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" class="w-full md:w-120" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService]
})
export class TreeBasicDemo implements OnInit {
    private nodeService = inject(NodeService);

    files = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## Checkbox

Selection of multiple nodes via checkboxes is enabled by configuring selectionMode as checkbox .

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" selectionMode="checkbox" class="w-full md:w-120" [(selection)]="selectedFiles" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService]
})
export class TreeCheckboxDemo implements OnInit {
    private nodeService = inject(NodeService);

    files = signal<TreeNode[]>(undefined);

    selectedFiles!: TreeNode[];

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## Content

Custom node content is rendered through the node template; the node exposes its children and expanded state so the right icon can be chosen.

```typescript
import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { FolderOpen } from '@primeicons/angular/folder-open';
import { Folder } from '@primeicons/angular/folder';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <p-tree [value]="nodes()" class="w-full md:w-120">
            <ng-template #node let-node>
                <span class="flex items-center gap-2">
                    @if (node.children?.length && node.expanded) {
                        <svg data-p-icon="folder-open" />
                    } @else if (node.children?.length) {
                        <svg data-p-icon="folder" />
                    } @else {
                        <svg data-p-icon="file" />
                    }
                    {{ node.label }}
                </span>
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule, FolderOpen, Folder, File],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeContentDemo implements OnInit {
    private nodeService = inject(NodeService);

    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            const stripIcons = (nodes: TreeNode[]) => {
                for (const node of nodes) {
                    delete node.icon;
                    if (node.children) stripIcons(node.children);
                }
            };
            stripIcons(data);
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }
}
```

## Context Menu

Tree has exclusive integration with ContextMenu using the contextMenu property along with the contextMenuSelection to manage the selection.

```typescript
import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { ContextMenu, ContextMenuModule } from 'primeng/contextmenu';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode, MenuItem, MessageService } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" class="w-full md:w-80" selectionMode="single" [(selection)]="selectedNode" [(contextMenuSelection)]="contextMenuNode" [contextMenu]="cm" />
        <p-contextmenu #cm [model]="items" />
    `,
    standalone: true,
    imports: [ContextMenuModule, TreeModule],
    providers: [NodeService, MessageService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeContextMenuDemo implements OnInit {
    private nodeService = inject(NodeService);

    private messageService = inject(MessageService);

    files = signal<TreeNode[]>([]);

    selectedNode = model<TreeNode | null>(null);

    contextMenuNode = model<TreeNode | null>(null);

    items!: MenuItem[];

    ngOnInit() {
        this.nodeService.getFiles().then((files) => this.files.set(files));
        this.items = [
            { label: 'View', icon: 'pi pi-search', command: () => this.viewFile(this.contextMenuNode()) },
            { label: 'Toggle', icon: 'pi pi-sort', command: () => this.toggleFile(this.contextMenuNode()) }
        ];
    }

    viewFile(node: TreeNode | null) {
        if (node) {
            this.messageService.add({ severity: 'info', summary: 'File Selected', detail: node.label });
        }
    }

    toggleFile(node: TreeNode | null) {
        if (node) {
            this.files.set(this.updateNodeInTree(this.files(), node.key, { ...node, expanded: !node.expanded }));
        }
    }

    updateNodeInTree(nodes: TreeNode[], key: string | undefined, updatedNode: TreeNode): TreeNode[] {
        return nodes.map((n) => {
            if (n.key === key) {
                return updatedNode;
            }

            if (n.children) {
                return { ...n, children: this.updateNodeInTree(n.children, key, updatedNode) };
            }

            return n;
        });
    }
}
```

## Controlled

Tree state can be controlled programmatically with the expanded property of the tree nodes that defines whether a node is open or closed.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Plus } from '@primeicons/angular/plus';
import { Minus } from '@primeicons/angular/minus';

@Component({
    template: `
        <div class="flex flex-wrap gap-2 mb-6">
            <button type="button" pButton (click)="expandAll()">
                <svg data-p-icon="plus" />
                Expand All
            </button>
            <button type="button" pButton (click)="collapseAll()">
                <svg data-p-icon="minus" />
                Collapse All
            </button>
        </div>
        <p-tree [value]="nodes()" class="w-full md:w-120" />
    `,
    standalone: true,
    imports: [TreeModule, ButtonModule, Plus, Minus],
    providers: [NodeService]
})
export class TreeControlledDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => this.nodes.set(data));
    }

    expandAll() {
        this.nodes.set(this.nodes().map((node) => this.toggleRecursive(node, true)));
    }

    collapseAll() {
        this.nodes.set(this.nodes().map((node) => this.toggleRecursive(node, false)));
    }

    private toggleRecursive(node: TreeNode, expanded: boolean): TreeNode {
        if (!node.children?.length) {
            return node;
        }

        return {
            ...node,
            expanded,
            children: node.children.map((child) => this.toggleRecursive(child, expanded))
        };
    }
}
```

## Empty

A custom placeholder can be displayed when there are no nodes to show using the empty template.

```typescript
import { Component, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { ButtonModule } from 'primeng/button';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <p-tree [value]="nodes()" class="w-full md:w-120">
            <ng-template #node let-node>
                <span class="flex items-center gap-2">
                    <svg data-p-icon="folder" />
                    {{ node.label }}
                </span>
            </ng-template>
            <ng-template #empty>
                <div class="flex flex-col items-center justify-center gap-3 py-10 text-center">
                    <div class="w-14 h-14 rounded-full bg-surface-100 dark:bg-surface-800 flex items-center justify-center">
                        <svg data-p-icon="folder" [size]="28" class="text-surface-400 dark:text-surface-500" />
                    </div>
                    <div>
                        <p class="m-0 font-semibold text-surface-900 dark:text-surface-0">No folders yet</p>
                        <p class="mt-1 text-sm text-surface-500 dark:text-surface-400">Create your first folder to start building a tree.</p>
                    </div>
                    <button type="button" pButton size="small" (click)="addNode()">
                        <svg data-p-icon="plus" />
                        New Folder
                    </button>
                </div>
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule, ButtonModule, Folder, Plus]
})
export class TreeEmptyDemo {
    nodes = signal<TreeNode[]>([]);

    addNode() {
        this.nodes.update((nodes) => [...nodes, { key: `root-${nodes.length}`, label: `New Folder ${nodes.length + 1}` }]);
    }
}
```

## Events

An event is provided for each type of user interaction such as expand, collapse and selection.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode, MessageService } from 'primeng/api';

@Component({
    template: `
        <p-tree
            [value]="files()"
            class="w-full md:w-120"
            selectionMode="single"
            [(selection)]="selectedFile"
            (onNodeExpand)="nodeExpand($event)"
            (onNodeCollapse)="nodeCollapse($event)"
            (onNodeSelect)="nodeSelect($event)"
            (onNodeUnselect)="nodeUnselect($event)"
        />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService, MessageService]
})
export class TreeEventDemo implements OnInit {
    private nodeService = inject(NodeService);

    private messageService = inject(MessageService);

    files = signal<TreeNode[]>(undefined);

    selectedFile!: TreeNode;

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }

    nodeExpand(event: any) {
        this.messageService.add({ severity: 'success', summary: 'Node Expanded', detail: event.node.label });
    }

    nodeCollapse(event: any) {
        this.messageService.add({ severity: 'warn', summary: 'Node Collapsed', detail: event.node.label });
    }

    nodeSelect(event: any) {
        this.messageService.add({ severity: 'info', summary: 'Node Selected', detail: event.node.label });
    }

    nodeUnselect(event: any) {
        this.messageService.add({ severity: 'info', summary: 'Node Unselected', detail: event.node.label });
    }
}
```

## Filter

Filtering enables searching through the nodes using the built-in filter property.

```typescript
import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" class="w-full md:w-120" [filter]="true" filterPlaceholder="Search" emptyMessage="No options found." [pt]="{ emptyMessage: { class: 'mt-2' } }" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeFilterDemo implements OnInit {
    files = signal<TreeNode[]>(undefined);

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## Keyboard

Tree has full keyboard support; arrow keys navigate between nodes, right and left expand and collapse a node and space toggles the selection of the focused node.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { BadgeModule } from 'primeng/badge';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Folder } from '@primeicons/angular/folder';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <div class="flex flex-col gap-3 w-full md:w-120">
            <div class="flex items-center justify-between gap-3 p-3 rounded-md border border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-900">
                <span class="text-sm text-surface-500 dark:text-surface-400">
                    <kbd class="px-1.5 py-0.5 text-xs rounded bg-surface-200 dark:bg-surface-700">↑</kbd>&nbsp;<kbd class="px-1.5 py-0.5 text-xs rounded bg-surface-200 dark:bg-surface-700">↓</kbd> navigate,&nbsp;<kbd
                        class="px-1.5 py-0.5 text-xs rounded bg-surface-200 dark:bg-surface-700"
                        >→</kbd
                    >
                    expand, <kbd class="px-1.5 py-0.5 text-xs rounded bg-surface-200 dark:bg-surface-700">←</kbd> collapse,&nbsp;<kbd class="px-1.5 py-0.5 text-xs rounded bg-surface-200 dark:bg-surface-700">Space</kbd> select
                </span>
                <div class="flex items-center gap-2">
                    <span class="text-sm font-medium">Selected</span>
                    <p-badge [value]="selectedCount" [severity]="selectedCount ? 'info' : 'secondary'" />
                </div>
            </div>
            <p-tree [value]="nodes()" selectionMode="multiple" [(selection)]="selection" class="w-full">
                <ng-template #node let-node>
                    <span class="flex items-center gap-2">
                        @if (node.children?.length) {
                            <svg data-p-icon="folder" />
                        } @else {
                            <svg data-p-icon="file" />
                        }
                        {{ node.label }}
                    </span>
                </ng-template>
            </p-tree>
        </div>
    `,
    standalone: true,
    imports: [BadgeModule, TreeModule, Folder, File],
    providers: [NodeService]
})
export class TreeKeyboardDemo implements OnInit {
    private nodeService = inject(NodeService);

    nodes = signal<TreeNode[]>(undefined);

    selection: TreeNode[] = [];

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }

    get selectedCount() {
        return this.selection?.length ?? 0;
    }
}
```

## Lazy

Lazy loading is demonstrated in this example where nodes are loaded on demand when a node is expanded.

```typescript
import { Component, OnInit, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { TreeNode } from 'primeng/api';
import { Spinner } from '@primeicons/angular/spinner';
import { MinusCircle } from '@primeicons/angular/minus-circle';
import { PlusCircle } from '@primeicons/angular/plus-circle';
import { File } from '@primeicons/angular/file';
import { FolderOpen } from '@primeicons/angular/folder-open';
import { Folder } from '@primeicons/angular/folder';

@Component({
    template: `
        <p-tree class="w-full md:w-120" [value]="nodes()" loadingMode="icon" (onNodeExpand)="onNodeExpand($event)">
            <ng-template #togglericon let-expanded let-loading="loading">
                @if (loading) {
                    <svg data-p-icon="spinner" class="animate-spin" />
                } @else if (expanded) {
                    <svg data-p-icon="minus-circle" />
                } @else {
                    <svg data-p-icon="plus-circle" />
                }
            </ng-template>
            <ng-template #node let-node>
                <span class="flex items-center">
                    @if (node.leaf) {
                        <svg data-p-icon="file" class="mr-2" />
                    } @else if (node.expanded) {
                        <svg data-p-icon="folder-open" class="mr-2" />
                    } @else {
                        <svg data-p-icon="folder" class="mr-2" />
                    }
                    <span>{{ node.label }}</span>
                </span>
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule, Spinner, MinusCircle, PlusCircle, File, FolderOpen, Folder],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeLazyDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodes.set(this.createLazyNodes());
        setTimeout(() => {
            this.nodes.set(this.nodes().map((node) => ({ ...node, loading: false })));
        }, 2000);
    }

    private createLazyNodes(): TreeNode[] {
        return [
            { key: '0', label: 'Node 0', leaf: false, loading: true },
            { key: '1', label: 'Node 1', leaf: false, loading: true },
            { key: '2', label: 'Node 2', leaf: false, loading: true }
        ];
    }

    onNodeExpand(event: any) {
        if (!event.node.children) {
            event.node.loading = true;

            setTimeout(() => {
                const _nodes = this.nodes();
                let _node = { ...event.node };

                _node.children = [
                    { key: `${_node.key}-0`, label: `Lazy ${_node.label} - 0` },
                    { key: `${_node.key}-1`, label: `Lazy ${_node.label} - 1` },
                    { key: `${_node.key}-2`, label: `Lazy ${_node.label} - 2` }
                ];
                _node.loading = false;

                const key = parseInt(_node.key, 10);

                _nodes[key] = _node;
                this.nodes.set([..._nodes]);
            }, 1000);
        }
    }
}
```

## Loading

While data is being fetched, a loading overlay with a spinner can be displayed over the Tree by enabling the loading property.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { Tree, TreeModule } from 'primeng/tree';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Refresh } from '@primeicons/angular/refresh';
import { Folder } from '@primeicons/angular/folder';
import { File } from '@primeicons/angular/file';
import { Spinner } from '@primeicons/angular/spinner';

@Component({
    template: `
        <div class="w-full md:w-120 flex flex-col gap-3">
            <button type="button" pButton size="small" [disabled]="loading()" class="self-end" (click)="refresh()">
                <svg data-p-icon="refresh" />
                Refresh
            </button>
            <p-tree [value]="nodes()" [loading]="loading()" class="w-full">
                <ng-template #node let-node>
                    <span class="flex items-center gap-2">
                        @if (node.children?.length) {
                            <svg data-p-icon="folder" />
                        } @else {
                            <svg data-p-icon="file" />
                        }
                        {{ node.label }}
                    </span>
                </ng-template>
                <ng-template #loadingicon>
                    <svg data-p-icon="spinner" [size]="32" class="animate-spin text-primary" />
                </ng-template>
            </p-tree>
        </div>
    `,
    standalone: true,
    imports: [TreeModule, ButtonModule, Refresh, Folder, File, Spinner],
    providers: [NodeService]
})
export class TreeLoadingDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    loading = signal(false);

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }

    refresh() {
        this.loading.set(true);
        setTimeout(() => this.loading.set(false), 2000);
    }
}
```

## loadingskeleton-doc

Skeleton placeholders can be rendered for each row while the data is being fetched by displaying a set of empty nodes with a custom node template.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { SkeletonModule } from 'primeng/skeleton';
import { TreeModule } from 'primeng/tree';
import { ButtonModule } from 'primeng/button';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { Refresh } from '@primeicons/angular/refresh';
import { Folder } from '@primeicons/angular/folder';
import { File } from '@primeicons/angular/file';

@Component({
    template: `
        <div class="w-full md:w-120 flex flex-col gap-3">
            <button type="button" pButton size="small" [disabled]="loading()" class="self-end" (click)="refresh()">
                <svg data-p-icon="refresh" />
                Refresh
            </button>
            <p-tree [value]="loading() ? placeholders : nodes()" class="w-full">
                <ng-template #node let-node>
                    <span class="flex items-center gap-2">
                        @if (loading()) {
                            <p-skeleton shape="circle" size="1rem" />
                            <p-skeleton width="10rem" borderRadius="4px" />
                        } @else {
                            @if (node.children?.length) {
                                <svg data-p-icon="folder" />
                            } @else {
                                <svg data-p-icon="file" />
                            }
                            {{ node.label }}
                        }
                    </span>
                </ng-template>
            </p-tree>
        </div>
    `,
    standalone: true,
    imports: [SkeletonModule, TreeModule, ButtonModule, Refresh, Folder, File],
    providers: [NodeService]
})
export class TreeLoadingSkeletonDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    loading = signal(false);

    placeholders: TreeNode[] = Array.from({ length: 6 }, (_, i) => ({ key: `skeleton-${i}`, label: '', leaf: true }));

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }

    refresh() {
        this.loading.set(true);
        setTimeout(() => this.loading.set(false), 2000);
    }
}
```

## Multiple

More than one node is selectable by setting selectionMode to multiple . By default in multiple selection mode, metaKey press (e.g. ⌘ ) is necessary to add to existing selections however this can be configured with disabling the metaKeySelection property. Note that in touch enabled devices, Tree always ignores metaKey. In multiple selection mode, value binding should be a key-value pair where key is the node key and value is a boolean to indicate selection.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { Tree, TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <div class="flex items-center mb-6 gap-2">
            <p-toggleswitch inputId="input-metakey" [(ngModel)]="metaKeySelection" />
            <label for="input-metakey">MetaKey</label>
        </div>
        <p-tree [metaKeySelection]="metaKeySelection" [value]="files()" class="w-full md:w-120" selectionMode="multiple" [(selection)]="selectedFiles" />
    `,
    standalone: true,
    imports: [ToggleSwitchModule, TreeModule, FormsModule],
    providers: [NodeService]
})
export class TreeMultipleDemo implements OnInit {
    private nodeService = inject(NodeService);

    metaKeySelection: boolean = false;

    files = signal<TreeNode[]>(undefined);

    selectedFiles!: TreeNode[];

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## node-doc

Tree nodes can be customized with the #node template.

```typescript
import { Component, OnInit, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="nodes()" class="w-full md:w-120">
            <ng-template #node let-node>
                @if (node.type === 'url') {
                    <a [href]="node.data" target="_blank" rel="noopener noreferrer" class="text-surface-700 dark:text-surface-100 hover:text-primary">{{ node.label }}</a>
                } @else {
                    <b>{{ node.label }}</b>
                }
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeNodeDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodes.set([
            {
                key: '0',
                label: 'Introduction',
                children: [
                    { key: '0-0', label: 'What is Angular', data: 'https://angular.io', type: 'url' },
                    { key: '0-1', label: 'Getting Started', data: 'https://angular.io/guide/setup-local', type: 'url' },
                    { key: '0-2', label: 'Learn and Explore', data: 'https://angular.io/guide/architecture', type: 'url' },
                    { key: '0-3', label: 'Take a Look', data: 'https://angular.io/start', type: 'url' }
                ]
            },
            {
                key: '1',
                label: 'Components In-Depth',
                children: [
                    { key: '1-0', label: 'Component Registration', data: 'https://angular.io/guide/component-interaction', type: 'url' },
                    { key: '1-1', label: 'User Input', data: 'https://angular.io/guide/user-input', type: 'url' },
                    { key: '1-2', label: 'Hooks', data: 'https://angular.io/guide/lifecycle-hooks', type: 'url' },
                    { key: '1-3', label: 'Attribute Directives', data: 'https://angular.io/guide/attribute-directives', type: 'url' }
                ]
            }
        ]);
    }
}
```

## preview-doc

```typescript
import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" class="w-full md:w-120" selectionMode="checkbox" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreePreviewDemo implements OnInit {
    files = signal<TreeNode[]>(undefined);

    private nodeService = inject(NodeService);

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## Select All

A header checkbox can be added to toggle the selection of every node at once. Its checked and indeterminate states are derived from the current selection.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="nodes()" selectionMode="checkbox" [(selection)]="selection" class="w-full md:w-120">
            <ng-template #header>
                <div class="flex items-center gap-2 px-2">
                    <p-checkbox [ngModel]="isAllSelected" [indeterminate]="isSomeSelected" [binary]="true" inputId="tree-select-all" (onChange)="toggleAll()" />
                    <label for="tree-select-all" class="font-medium">Select All</label>
                </div>
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [CheckboxModule, TreeModule, FormsModule],
    providers: [NodeService]
})
export class TreeSelectAllDemo implements OnInit {
    private nodeService = inject(NodeService);

    nodes = signal<TreeNode[]>(undefined);

    selection: TreeNode[] = [];

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }

    private flatten(list: TreeNode[] = []): TreeNode[] {
        return list.reduce<TreeNode[]>((acc, node) => {
            acc.push(node);
            if (node.children) acc.push(...this.flatten(node.children));

            return acc;
        }, []);
    }

    get totalCount() {
        return this.flatten(this.nodes()).length;
    }

    get isAllSelected() {
        return this.totalCount > 0 && this.selection.length === this.totalCount;
    }

    get isSomeSelected() {
        return this.selection.length > 0 && !this.isAllSelected;
    }

    toggleAll() {
        this.selection = this.isAllSelected ? [] : this.flatten(this.nodes());
    }
}
```

## Single

Single node selection is configured by setting selectionMode as single along with selection properties to manage the selection value binding.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="files()" class="w-full md:w-120" selectionMode="single" [(selection)]="selectedFile" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService]
})
export class TreeSingleDemo implements OnInit {
    private nodeService = inject(NodeService);

    files = signal<TreeNode[]>(undefined);

    selectedFile!: TreeNode;

    ngOnInit() {
        this.nodeService.getFiles().then((data) => {
            this.files.set(data);
        });
    }
}
```

## template-doc

Custom node content instead of a node label is defined with the #node template reference.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="nodes()" class="w-full md:w-120">
            <ng-template #node let-node>
                @if (node.type === 'url') {
                    <a [href]="node.data" target="_blank" rel="noopener noreferrer" class="text-surface-700 dark:text-surface-100 hover:text-primary">{{ node.label }}</a>
                } @else {
                    <b>{{ node.label }}</b>
                }
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule]
})
export class TreeTemplateDemo implements OnInit {
    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodes.set([
            {
                key: '0',
                label: 'Introduction',
                children: [
                    { key: '0-0', label: 'What is Angular', data: 'https://angular.io', type: 'url' },
                    { key: '0-1', label: 'Getting Started', data: 'https://angular.io/guide/setup-local', type: 'url' },
                    { key: '0-2', label: 'Learn and Explore', data: 'https://angular.io/guide/architecture', type: 'url' },
                    { key: '0-3', label: 'Take a Look', data: 'https://angular.io/start', type: 'url' }
                ]
            },
            {
                key: '1',
                label: 'Components In-Depth',
                children: [
                    { key: '1-0', label: 'Component Registration', data: 'https://angular.io/guide/component-interaction', type: 'url' },
                    { key: '1-1', label: 'User Input', data: 'https://angular.io/guide/user-input', type: 'url' },
                    { key: '1-2', label: 'Hooks', data: 'https://angular.io/guide/lifecycle-hooks', type: 'url' },
                    { key: '1-3', label: 'Attribute Directives', data: 'https://angular.io/guide/attribute-directives', type: 'url' }
                ]
            }
        ]);
    }
}
```

## Toggle Indicator

Toggle icons are customized through the togglericon template which receives the expanded state of the node.

```typescript
import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';
import { MinusCircle } from '@primeicons/angular/minus-circle';
import { PlusCircle } from '@primeicons/angular/plus-circle';

@Component({
    template: `
        <p-tree [value]="nodes()" class="w-full md:w-120">
            <ng-template #togglericon let-expanded>
                @if (expanded) {
                    <svg data-p-icon="minus-circle" />
                } @else {
                    <svg data-p-icon="plus-circle" />
                }
            </ng-template>
        </p-tree>
    `,
    standalone: true,
    imports: [TreeModule, MinusCircle, PlusCircle],
    providers: [NodeService],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TreeToggleIndicatorDemo implements OnInit {
    private nodeService = inject(NodeService);

    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodeService.getTreeNodes().then((data: TreeNode[]) => {
            data[0].expanded = true;
            this.nodes.set(data);
        });
    }
}
```

## Virtual Scroll

VirtualScroller is a performance-approach to handle huge data efficiently. Setting virtualScroll property as true and providing a virtualScrollItemSize in pixels would be enough to enable this functionality.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="nodes()" scrollHeight="250px" [virtualScroll]="true" [virtualScrollItemSize]="35" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService]
})
export class TreeVirtualScrollDemo implements OnInit {
    private nodeService = inject(NodeService);

    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.nodes.set(this.nodeService.generateNodes(150));
    }
}
```

## virtualscrolllazy-doc

VirtualScroller is a performance-approach to handle huge data efficiently. Setting virtualScroll property as true and providing a virtualScrollItemSize in pixels would be enough to enable this functionality.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { TreeModule } from 'primeng/tree';
import { NodeService } from '@/service/nodeservice';
import { TreeNode } from 'primeng/api';

@Component({
    template: `
        <p-tree [value]="nodes()" scrollHeight="250px" [virtualScroll]="true" [lazy]="true" [virtualScrollItemSize]="35" (onNodeExpand)="nodeExpand($event)" [loading]="loading()" />
    `,
    standalone: true,
    imports: [TreeModule],
    providers: [NodeService]
})
export class TreeVirtualScrollLazyDemo implements OnInit {
    private nodeService = inject(NodeService);

    loading = signal<boolean>(false);

    nodes = signal<TreeNode[]>(undefined);

    ngOnInit() {
        this.loading.set(true);
        setTimeout(() => {
            this.nodes.set(this.nodeService.generateNodes(150));
            this.loading.set(false);
        }, 1000);
    }

    nodeExpand(event: any) {
        if (event.node) {
            this.loading.set(true);
            setTimeout(() => {
                event.node.children = this.nodeService.createNodes(5, event.node.key);
                this.loading.set(false);
                this.nodes.set([...this.nodes()]);
            }, 200);
        }
    }
}
```

## Tree

Tree is used to display hierarchical data.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, TreePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | any | - | An array of treenodes. |
| selectionMode | "single" \| "multiple" \| "checkbox" \| null \| undefined | - | Defines the selection mode. |
| loadingMode | "mask" \| "icon" | - | Loading mode display. |
| selection | TreeNode<any> \| TreeNode<any>[] \| null \| undefined | - | A single treenode instance or an array to refer to the selections. **(Deprecated)** |
| collapsedKeys | { [key: string]: boolean } \| null | - | A map of node keys (by  `TreeNode.key` ) whose value is  `true`  for collapsed nodes. Two-way bindable. When provided it controls expansion (a node is expanded unless its key maps to  `true` ); leave it unset to keep driving expansion from each node's  `expanded`  flag (default, non-breaking). |
| selectionKeys | { [key: string]: any } \| null | - | A map of node keys (by  `TreeNode.key` ) describing the selection state. Two-way bindable. For single/multiple selection a key maps to  `true` ; for checkbox selection it maps to  `{ checked, partialChecked }` . When provided it is kept in sync with  `selection` ; leave it unset to keep using the node-reference based  `selection`  model (default, non-breaking). |
| contextMenu | any | - | Context menu instance. |
| contextMenuSelection | TreeNode<any> \| null | - | Selected node with a context menu. |
| draggableScope | any | - | Scope of the draggable nodes to match a droppableScope. |
| droppableScope | any | - | Scope of the droppable nodes to match a draggableScope. |
| draggableNodes | boolean | - | Whether the nodes are draggable. |
| droppableNodes | boolean | - | Whether the nodes are droppable. |
| metaKeySelection | boolean | - | Defines how multiple items can be selected, when true metaKey needs to be pressed to select or unselect an item and when set to false selection of each item can be toggled individually. On touch enabled devices, metaKeySelection is turned off automatically. |
| propagateSelectionUp | boolean | - | Whether checkbox selections propagate to ancestor nodes. |
| propagateSelectionDown | boolean | - | Whether checkbox selections propagate to descendant nodes. |
| loading | boolean | - | Displays a loader to indicate data load is in progress. |
| loadingIcon | string \| undefined | - | The icon to show while indicating data load is in progress. |
| emptyMessage | string | - | Text to display when there is no data. |
| ariaLabel | string \| undefined | - | Used to define a string that labels the tree. |
| togglerAriaLabel | string \| undefined | - | Defines a string that labels the toggler icon for accessibility. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| validateDrop | boolean | - | When enabled, drop can be accepted or rejected based on condition defined at onNodeDrop. |
| filter | boolean | - | When specified, displays an input field to filter the items. |
| filterInputAutoFocus | boolean | - | Determines whether the filter input should be automatically focused when the component is rendered. |
| filterBy | string | - | When filtering is enabled, filterBy decides which field or fields (comma separated) to search against. |
| filterMode | string | - | Mode for filtering valid values are "lenient" and "strict". Default is lenient. |
| filterOptions | any | - | Mode for filtering valid values are "lenient" and "strict". Default is lenient. |
| filterPlaceholder | string \| undefined | - | Placeholder text to show when filter input is empty. |
| filteredNodes | TreeNode<any>[] \| null \| undefined | - | Values after the tree nodes are filtered. |
| filterLocale | string \| undefined | - | Locale to use in filtering. The default locale is the host environment's current locale. |
| scrollHeight | string \| undefined | - | Height of the scrollable viewport. |
| lazy | boolean | - | Defines if data is loaded and interacted with in lazy manner. |
| virtualScroll | boolean | - | Whether the data should be loaded on demand during scroll. |
| virtualScrollItemSize | number \| undefined | - | Height of an item in the list for VirtualScrolling. |
| virtualScrollOptions | ScrollerOptions \| undefined | - | Whether to use the scroller feature. The properties of scroller component can be used like an object in it. |
| indentation | number | - | Indentation factor for spacing of the nested node when virtual scrolling is enabled. |
| trackBy | Function | - | Function to optimize the node list rendering, default algorithm checks for object identity. |
| highlightOnSelect | boolean | - | Highlights the node on select. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onNodeSelect | event: TreeNodeSelectEvent | Callback to invoke when a node is selected. |
| onNodeUnselect | event: TreeNodeUnSelectEvent | Callback to invoke when a node is unselected. |
| onNodeExpand | event: TreeNodeExpandEvent | Callback to invoke when a node is expanded. |
| onNodeCollapse | event: TreeNodeCollapseEvent | Callback to invoke when a node is collapsed. |
| onNodeContextMenuSelect | event: TreeNodeContextMenuSelectEvent | Callback to invoke when a node is selected with right click. |
| onNodeDoubleClick | event: TreeNodeDoubleClickEvent | Callback to invoke when a node is double clicked. |
| onNodeDrop | event: TreeNodeDropEvent | Callback to invoke when a node is dropped. |
| onLazyLoad | event: TreeLazyLoadEvent | Callback to invoke in lazy mode to load new data. |
| onScroll | event: TreeScrollEvent | Callback to invoke in virtual scroll mode when scroll position changes. |
| onScrollIndexChange | event: TreeScrollIndexChangeEvent | Callback to invoke in virtual scroll mode when scroll position and item's range in view changes. |
| onFilter | event: TreeFilterEvent | Callback to invoke when data is filtered. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| filter | TemplateRef<TreeFilterTemplateContext> \| undefined | Custom filter template. |
| node | TemplateRef<any> \| undefined | Custom node template. |
| header | TemplateRef<void> \| undefined | Custom header template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| loader | TemplateRef<TreeLoaderTemplateContext> \| undefined | Custom loader template. |
| empty | TemplateRef<void> \| undefined | Custom empty message template. |
| togglericon | TemplateRef<TreeTogglerIconTemplateContext> \| undefined | Custom toggler icon template. |
| checkboxicon | TemplateRef<TreeCheckboxIconTemplateContext> \| undefined | Custom checkbox icon template. |
| loadingicon | TemplateRef<void> \| undefined | Custom loading icon template. |
| filtericon | TemplateRef<void> \| undefined | Custom filter icon template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| resetFilter |  | void | Resets filter. |
| scrollToVirtualIndex | index: number | void | Scrolls to virtual index. |
| scrollTo | options: any | void | Scrolls to virtual index. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| mask | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the loading mask's DOM element. |
| loadingIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the loading icon's DOM element. |
| pcFilterContainer | IconFieldPassThrough | Used to pass attributes to the filter container's DOM element. |
| pcFilterIconContainer | InputIconPassThrough | Used to pass attributes to the filter icon container's DOM element. |
| pcFilterInput | InputTextPassThrough | Used to pass attributes to the filter input's DOM element. |
| filterIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the filter icon's DOM element. |
| pcScroller | VirtualScrollerPassThrough | Used to pass attributes to the Scroller component. |
| wrapper | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the wrapper's DOM element. |
| rootChildren | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the root children's DOM element. |
| node | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the node's DOM element. |
| dropPoint | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the drop point's DOM element. |
| nodeContent | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the node content's DOM element. |
| nodeToggleButton | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the node toggle button's DOM element. |
| nodeTogglerIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the node toggle icon's DOM element. |
| pcNodeCheckbox | CheckboxPassThrough | Used to pass attributes to the node checkbox's DOM element. |
| nodeIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the node icon's DOM element. |
| nodeLabel | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the node label's DOM element. |
| nodeChildren | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the node children's DOM element. |
| emptyMessage | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the empty message's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-tree | Class name of the root element |
| p-tree-mask | Class name of the mask element |
| p-tree-loading-icon | Class name of the loading icon element |
| p-tree-filter-input | Class name of the filter input element |
| p-tree-root | Class name of the wrapper element |
| p-tree-root-children | Class name of the root children element |
| p-tree-node | Class name of the node element |
| p-tree-node-content | Class name of the node content element |
| p-tree-node-toggle-button | Class name of the node toggle button element |
| p-tree-node-toggle-icon | Class name of the node toggle icon element |
| p-tree-node-checkbox | Class name of the node checkbox element |
| p-tree-node-icon | Class name of the node icon element |
| p-tree-node-label | Class name of the node label element |
| p-tree-node-children | Class name of the node children element |
| p-tree-empty-message | Class name of the empty message element |
| p-tree-node-droppoint | Class name of the drop point element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| tree.background | --p-tree-background | Background of root |
| tree.color | --p-tree-color | Color of root |
| tree.padding | --p-tree-padding | Padding of root |
| tree.gap | --p-tree-gap | Gap of root |
| tree.indent | --p-tree-indent | Indent of root |
| tree.transition.duration | --p-tree-transition-duration | Transition duration of root |
| tree.node.padding | --p-tree-node-padding | Padding of node |
| tree.node.border.radius | --p-tree-node-border-radius | Border radius of node |
| tree.node.hover.background | --p-tree-node-hover-background | Hover background of node |
| tree.node.selected.background | --p-tree-node-selected-background | Selected background of node |
| tree.node.color | --p-tree-node-color | Color of node |
| tree.node.hover.color | --p-tree-node-hover-color | Hover color of node |
| tree.node.selected.color | --p-tree-node-selected-color | Selected color of node |
| tree.node.focus.ring.width | --p-tree-node-focus-ring-width | Focus ring width of node |
| tree.node.focus.ring.style | --p-tree-node-focus-ring-style | Focus ring style of node |
| tree.node.focus.ring.color | --p-tree-node-focus-ring-color | Focus ring color of node |
| tree.node.focus.ring.offset | --p-tree-node-focus-ring-offset | Focus ring offset of node |
| tree.node.focus.ring.shadow | --p-tree-node-focus-ring-shadow | Focus ring shadow of node |
| tree.node.gap | --p-tree-node-gap | Gap of node |
| tree.node.icon.color | --p-tree-node-icon-color | Color of node icon |
| tree.node.icon.hover.color | --p-tree-node-icon-hover-color | Hover color of node icon |
| tree.node.icon.selected.color | --p-tree-node-icon-selected-color | Selected color of node icon |
| tree.node.label.font.weight | --p-tree-node-label-font-weight | Font weight of node label |
| tree.node.label.font.size | --p-tree-node-label-font-size | Font size of node label |
| tree.node.label.selected.font.weight | --p-tree-node-label-selected-font-weight | Font weight of a selected node label |
| tree.node.toggle.button.border.radius | --p-tree-node-toggle-button-border-radius | Border radius of node toggle button |
| tree.node.toggle.button.size | --p-tree-node-toggle-button-size | Size of node toggle button |
| tree.node.toggle.button.hover.background | --p-tree-node-toggle-button-hover-background | Hover background of node toggle button |
| tree.node.toggle.button.selected.hover.background | --p-tree-node-toggle-button-selected-hover-background | Selected hover background of node toggle button |
| tree.node.toggle.button.color | --p-tree-node-toggle-button-color | Color of node toggle button |
| tree.node.toggle.button.hover.color | --p-tree-node-toggle-button-hover-color | Hover color of node toggle button |
| tree.node.toggle.button.selected.hover.color | --p-tree-node-toggle-button-selected-hover-color | Selected hover color of node toggle button |
| tree.node.toggle.button.focus.ring.width | --p-tree-node-toggle-button-focus-ring-width | Focus ring width of node toggle button |
| tree.node.toggle.button.focus.ring.style | --p-tree-node-toggle-button-focus-ring-style | Focus ring style of node toggle button |
| tree.node.toggle.button.focus.ring.color | --p-tree-node-toggle-button-focus-ring-color | Focus ring color of node toggle button |
| tree.node.toggle.button.focus.ring.offset | --p-tree-node-toggle-button-focus-ring-offset | Focus ring offset of node toggle button |
| tree.node.toggle.button.focus.ring.shadow | --p-tree-node-toggle-button-focus-ring-shadow | Focus ring shadow of node toggle button |
| tree.loading.icon.size | --p-tree-loading-icon-size | Size of loading icon |
| tree.filter.margin | --p-tree-filter-margin | Margin of filter |
