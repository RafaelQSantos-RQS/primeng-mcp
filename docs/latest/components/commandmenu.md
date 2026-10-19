---
title: "Angular CommandMenu Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/commandmenu.md"
description: "CommandMenu is a search-driven command palette component."
---

# Angular CommandMenu Component

CommandMenu is a search-driven command palette component.

## Accessibility

Screen Reader The search input has a combobox role with aria-expanded , aria-controls and aria-activedescendant attributes. The list element uses the listbox role and each item has an option role. Value to describe the component can be provided via ariaLabel prop.

## Basic

CommandMenu requires a collection of menuitems via the model property. Grouped commands are defined using the items property of a menuitem.

```typescript
import { Component } from '@angular/core';
import { CommandMenuModule } from 'primeng/commandmenu';
import { MenuItem } from 'primeng/api';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <p-commandmenu [model]="commands" placeholder="Search for commands..." class="mx-auto">
            <ng-template #emptyMessage>No results found</ng-template>
            <ng-template #footer>
                <div class="flex items-center justify-end gap-3 w-full">
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-up"></svg>
                        </kbd>
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-down"></svg>
                        </kbd>
                        Navigate
                    </span>
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700"> &#x21B5; </kbd>
                        Select
                    </span>
                </div>
            </ng-template>
        </p-commandmenu>
    `,
    standalone: true,
    imports: [CommandMenuModule, ArrowUp, ArrowDown]
})
export class CommandMenuBasicDemo {
    commands: MenuItem[] = [
        {
            label: 'Recents',
            items: [{ label: 'Check For Updates', keywords: ['check', 'updates'] }, { label: 'Open Settings' }, { label: 'Search Files' }, { label: 'Open Terminal' }, { label: 'View History', keywords: ['history', 'recent'] }, { label: 'Open Chat' }]
        },
        {
            label: 'Files',
            items: [
                { label: 'New File' },
                { label: 'New Folder' },
                { label: 'Save All' },
                { label: 'Change Theme' },
                { label: 'Run Task' },
                { label: 'Stop Task' },
                { label: 'Export Project' },
                { label: 'Import Project' },
                { label: 'Delete File' },
                { label: 'Duplicate File' }
            ]
        },
        {
            label: 'Source',
            items: [{ label: 'Git: Commit' }, { label: 'Git: Push' }, { label: 'Git: Pull' }, { label: 'Switch Account' }, { label: 'Open Documentation' }, { label: 'Git: Sync' }, { label: 'Git: Create Branch' }, { label: 'Git: Create Tag' }]
        },
        {
            label: 'Editor',
            items: [{ label: 'Align Left' }, { label: 'Align Center' }, { label: 'Align Right' }, { label: 'Toggle Bold' }, { label: 'Toggle Italic' }, { label: 'Insert Link' }, { label: 'Insert Image' }, { label: 'Insert List' }]
        },
        {
            label: 'Navigation',
            items: [{ label: 'Go to Home' }, { label: 'Go Back' }, { label: 'Go Forward' }, { label: 'Open Explorer' }, { label: 'View Bookmarks' }, { label: 'Open Minimap' }]
        },
        {
            label: 'View',
            items: [{ label: 'Toggle Preview' }, { label: 'Maximize Window' }, { label: 'Minimize Window' }, { label: 'Grid View' }, { label: 'List View' }, { label: 'Light Mode' }, { label: 'Dark Mode' }]
        },
        {
            label: 'Tools',
            items: [{ label: 'Open Calculator' }, { label: 'Open Calendar' }, { label: 'Open Timer' }, { label: 'View Analytics' }, { label: 'View Trends' }, { label: 'Open Database' }]
        }
    ];
}
```

## Controlled

The search value can be controlled with two-way binding using [(search)] . An empty template customizes the message when no results are found.

```typescript
import { Component } from '@angular/core';
import { CommandMenuModule } from 'primeng/commandmenu';
import { MenuItem } from 'primeng/api';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <p-commandmenu [model]="commands" [(search)]="searchValue" placeholder="Search for commands..." class="mx-auto">
            <ng-template #emptyMessage let-search>
                No results found for <span class="text-surface-900 dark:text-surface-0">"{{ search }}"</span>
            </ng-template>
            <ng-template #footer>
                <div class="flex items-center justify-end gap-3 w-full">
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-up"></svg>
                        </kbd>
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-down"></svg>
                        </kbd>
                        Navigate
                    </span>
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700"> &#x21B5; </kbd>
                        Select
                    </span>
                </div>
            </ng-template>
        </p-commandmenu>
    `,
    standalone: true,
    imports: [CommandMenuModule, ArrowUp, ArrowDown]
})
export class CommandMenuControlledDemo {
    searchValue: string = '';

    commands: MenuItem[] = [
        {
            label: 'Recents',
            items: [{ label: 'Check For Updates', keywords: ['check', 'updates'] }, { label: 'Open Settings' }, { label: 'Search Files' }, { label: 'Open Terminal' }, { label: 'View History', keywords: ['history', 'recent'] }, { label: 'Open Chat' }]
        },
        {
            label: 'Files',
            items: [
                { label: 'New File' },
                { label: 'New Folder' },
                { label: 'Save All' },
                { label: 'Change Theme' },
                { label: 'Run Task' },
                { label: 'Stop Task' },
                { label: 'Export Project' },
                { label: 'Import Project' },
                { label: 'Delete File' },
                { label: 'Duplicate File' }
            ]
        },
        {
            label: 'Source',
            items: [{ label: 'Git: Commit' }, { label: 'Git: Push' }, { label: 'Git: Pull' }, { label: 'Switch Account' }, { label: 'Open Documentation' }, { label: 'Git: Sync' }, { label: 'Git: Create Branch' }, { label: 'Git: Create Tag' }]
        },
        {
            label: 'Editor',
            items: [{ label: 'Align Left' }, { label: 'Align Center' }, { label: 'Align Right' }, { label: 'Toggle Bold' }, { label: 'Toggle Italic' }, { label: 'Insert Link' }, { label: 'Insert Image' }, { label: 'Insert List' }]
        },
        {
            label: 'Navigation',
            items: [{ label: 'Go to Home' }, { label: 'Go Back' }, { label: 'Go Forward' }, { label: 'Open Explorer' }, { label: 'View Bookmarks' }, { label: 'Open Minimap' }]
        },
        {
            label: 'View',
            items: [{ label: 'Toggle Preview' }, { label: 'Maximize Window' }, { label: 'Minimize Window' }, { label: 'Grid View' }, { label: 'List View' }, { label: 'Light Mode' }, { label: 'Dark Mode' }]
        },
        {
            label: 'Tools',
            items: [{ label: 'Open Calculator' }, { label: 'Open Calendar' }, { label: 'Open Timer' }, { label: 'View Analytics' }, { label: 'View Trends' }, { label: 'Open Database' }]
        }
    ];
}
```

## Custom

Custom content can be displayed for each item using the item template.

```typescript
import { Component } from '@angular/core';
import { CommandMenuModule } from 'primeng/commandmenu';
import { MenuItem } from 'primeng/api';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <p-commandmenu [model]="commands" placeholder="Search for commands..." class="mx-auto">
            <ng-template #item let-item>
                <div class="p-menu-item-link flex items-center gap-3.5 w-full">
                    <span [class]="'w-5 h-5 rounded-md flex items-center justify-center text-white ' + item.color">
                        <i [class]="'pi ' + item.icon + ' text-xs font-bold'"></i>
                    </span>
                    <span class="text-sm">{{ item.label }}</span>
                    <span class="opacity-50 ml-auto text-sm">{{ item.category }}</span>
                </div>
            </ng-template>
            <ng-template #emptyMessage>No results found</ng-template>
            <ng-template #footer>
                <div class="flex items-center justify-end gap-3 w-full">
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-up"></svg>
                        </kbd>
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-down"></svg>
                        </kbd>
                        Navigate
                    </span>
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700"> &#x21B5; </kbd>
                        Select
                    </span>
                </div>
            </ng-template>
        </p-commandmenu>
    `,
    standalone: true,
    imports: [CommandMenuModule, ArrowUp, ArrowDown]
})
export class CommandMenuCustomDemo {
    commands: MenuItem[] = [
        {
            label: 'Recents',
            items: [
                { icon: 'pi-refresh', label: 'Check For Updates', category: 'Command', color: 'bg-[linear-gradient(rgb(245,83,84),rgb(235,70,70))]', keywords: ['check', 'updates'] },
                { icon: 'pi-cog', label: 'Open Settings', category: 'Command', color: 'bg-[linear-gradient(rgb(96,165,250),rgb(59,130,246))]' },
                { icon: 'pi-search', label: 'Search Files', category: 'Command', color: 'bg-[linear-gradient(rgb(167,139,250),rgb(139,92,246))]' },
                { icon: 'pi-terminal', label: 'Open Terminal', category: 'View', color: 'bg-[linear-gradient(rgb(148,163,184),rgb(100,116,139))]' },
                { icon: 'pi-history', label: 'View History', category: 'View', color: 'bg-[linear-gradient(rgb(192,132,252),rgb(168,85,247))]', keywords: ['history', 'recent'] },
                { icon: 'pi-comments', label: 'Open Chat', category: 'Communication', color: 'bg-[linear-gradient(rgb(34,211,238),rgb(6,182,212))]' }
            ]
        },
        {
            label: 'Files',
            items: [
                { icon: 'pi-file', label: 'New File', category: 'File', color: 'bg-[linear-gradient(rgb(52,211,153),rgb(16,185,129))]' },
                { icon: 'pi-folder', label: 'New Folder', category: 'File', color: 'bg-[linear-gradient(rgb(251,191,36),rgb(245,158,11))]' },
                { icon: 'pi-save', label: 'Save All', category: 'File', color: 'bg-[linear-gradient(rgb(34,197,94),rgb(22,163,74))]' },
                { icon: 'pi-palette', label: 'Change Theme', category: 'Appearance', color: 'bg-[linear-gradient(rgb(251,146,60),rgb(249,115,22))]' },
                { icon: 'pi-play', label: 'Run Task', category: 'Command', color: 'bg-[linear-gradient(rgb(34,197,94),rgb(21,128,61))]' },
                { icon: 'pi-stop', label: 'Stop Task', category: 'Command', color: 'bg-[linear-gradient(rgb(239,68,68),rgb(220,38,38))]' },
                { icon: 'pi-file-export', label: 'Export Project', category: 'File', color: 'bg-[linear-gradient(rgb(147,51,234),rgb(126,34,206))]' },
                { icon: 'pi-file-import', label: 'Import Project', category: 'File', color: 'bg-[linear-gradient(rgb(99,102,241),rgb(79,70,229))]' },
                { icon: 'pi-trash', label: 'Delete File', category: 'File', color: 'bg-[linear-gradient(rgb(239,68,68),rgb(185,28,28))]' },
                { icon: 'pi-copy', label: 'Duplicate File', category: 'File', color: 'bg-[linear-gradient(rgb(156,163,175),rgb(107,114,128))]' }
            ]
        },
        {
            label: 'Source',
            items: [
                { icon: 'pi-git', label: 'Git: Commit', category: 'Source Control', color: 'bg-[linear-gradient(rgb(249,115,22),rgb(234,88,12))]' },
                { icon: 'pi-upload', label: 'Git: Push', category: 'Source Control', color: 'bg-[linear-gradient(rgb(14,165,233),rgb(2,132,199))]' },
                { icon: 'pi-download', label: 'Git: Pull', category: 'Source Control', color: 'bg-[linear-gradient(rgb(59,130,246),rgb(37,99,235))]' },
                { icon: 'pi-users', label: 'Switch Account', category: 'Account', color: 'bg-[linear-gradient(rgb(236,72,153),rgb(219,39,119))]' },
                { icon: 'pi-book', label: 'Open Documentation', category: 'Help', color: 'bg-[linear-gradient(rgb(147,197,253),rgb(96,165,250))]' },
                { icon: 'pi-sync', label: 'Git: Sync', category: 'Source Control', color: 'bg-[linear-gradient(rgb(74,222,128),rgb(34,197,94))]' },
                { icon: 'pi-code-branch', label: 'Git: Create Branch', category: 'Source Control', color: 'bg-[linear-gradient(rgb(251,146,60),rgb(249,115,22))]' },
                { icon: 'pi-tag', label: 'Git: Create Tag', category: 'Source Control', color: 'bg-[linear-gradient(rgb(196,181,253),rgb(167,139,250))]' }
            ]
        },
        {
            label: 'Editor',
            items: [
                { icon: 'pi-align-left', label: 'Align Left', category: 'Editor', color: 'bg-[linear-gradient(rgb(147,197,253),rgb(59,130,246))]' },
                { icon: 'pi-align-center', label: 'Align Center', category: 'Editor', color: 'bg-[linear-gradient(rgb(147,197,253),rgb(59,130,246))]' },
                { icon: 'pi-align-right', label: 'Align Right', category: 'Editor', color: 'bg-[linear-gradient(rgb(147,197,253),rgb(59,130,246))]' },
                { icon: 'pi-bold', label: 'Toggle Bold', category: 'Editor', color: 'bg-[linear-gradient(rgb(30,41,59),rgb(15,23,42))]' },
                { icon: 'pi-italic', label: 'Toggle Italic', category: 'Editor', color: 'bg-[linear-gradient(rgb(71,85,105),rgb(51,65,85))]' },
                { icon: 'pi-link', label: 'Insert Link', category: 'Editor', color: 'bg-[linear-gradient(rgb(59,130,246),rgb(37,99,235))]' },
                { icon: 'pi-image', label: 'Insert Image', category: 'Editor', color: 'bg-[linear-gradient(rgb(168,85,247),rgb(147,51,234))]' },
                { icon: 'pi-list', label: 'Insert List', category: 'Editor', color: 'bg-[linear-gradient(rgb(34,197,94),rgb(22,163,74))]' }
            ]
        },
        {
            label: 'Navigation',
            items: [
                { icon: 'pi-home', label: 'Go to Home', category: 'Navigation', color: 'bg-[linear-gradient(rgb(96,165,250),rgb(59,130,246))]' },
                { icon: 'pi-arrow-left', label: 'Go Back', category: 'Navigation', color: 'bg-[linear-gradient(rgb(148,163,184),rgb(100,116,139))]' },
                { icon: 'pi-arrow-right', label: 'Go Forward', category: 'Navigation', color: 'bg-[linear-gradient(rgb(148,163,184),rgb(100,116,139))]' },
                { icon: 'pi-compass', label: 'Open Explorer', category: 'Navigation', color: 'bg-[linear-gradient(rgb(251,191,36),rgb(245,158,11))]' },
                { icon: 'pi-bookmark', label: 'View Bookmarks', category: 'Navigation', color: 'bg-[linear-gradient(rgb(249,115,22),rgb(234,88,12))]' },
                { icon: 'pi-map', label: 'Open Minimap', category: 'Navigation', color: 'bg-[linear-gradient(rgb(52,211,153),rgb(16,185,129))]' }
            ]
        },
        {
            label: 'View',
            items: [
                { icon: 'pi-eye', label: 'Toggle Preview', category: 'View', color: 'bg-[linear-gradient(rgb(147,51,234),rgb(126,34,206))]' },
                { icon: 'pi-window-maximize', label: 'Maximize Window', category: 'View', color: 'bg-[linear-gradient(rgb(100,116,139),rgb(71,85,105))]' },
                { icon: 'pi-window-minimize', label: 'Minimize Window', category: 'View', color: 'bg-[linear-gradient(rgb(148,163,184),rgb(100,116,139))]' },
                { icon: 'pi-th-large', label: 'Grid View', category: 'View', color: 'bg-[linear-gradient(rgb(34,197,94),rgb(22,163,74))]' },
                { icon: 'pi-bars', label: 'List View', category: 'View', color: 'bg-[linear-gradient(rgb(59,130,246),rgb(37,99,235))]' },
                { icon: 'pi-sun', label: 'Light Mode', category: 'View', color: 'bg-[linear-gradient(rgb(253,224,71),rgb(250,204,21))]' },
                { icon: 'pi-moon', label: 'Dark Mode', category: 'View', color: 'bg-[linear-gradient(rgb(30,41,59),rgb(15,23,42))]' }
            ]
        },
        {
            label: 'Tools',
            items: [
                { icon: 'pi-calculator', label: 'Open Calculator', category: 'Tools', color: 'bg-[linear-gradient(rgb(148,163,184),rgb(100,116,139))]' },
                { icon: 'pi-calendar', label: 'Open Calendar', category: 'Tools', color: 'bg-[linear-gradient(rgb(96,165,250),rgb(59,130,246))]' },
                { icon: 'pi-clock', label: 'Open Timer', category: 'Tools', color: 'bg-[linear-gradient(rgb(251,146,60),rgb(249,115,22))]' },
                { icon: 'pi-chart-bar', label: 'View Analytics', category: 'Tools', color: 'bg-[linear-gradient(rgb(34,197,94),rgb(22,163,74))]' },
                { icon: 'pi-chart-line', label: 'View Trends', category: 'Tools', color: 'bg-[linear-gradient(rgb(59,130,246),rgb(37,99,235))]' },
                { icon: 'pi-database', label: 'Open Database', category: 'Tools', color: 'bg-[linear-gradient(rgb(168,85,247),rgb(147,51,234))]' }
            ]
        }
    ];
}
```

## Filter

A custom filter function can be provided with the filter property. The function receives the item and search term, and should return a score (0 means no match).

```typescript
import { Component } from '@angular/core';
import { CommandMenuModule } from 'primeng/commandmenu';
import { MenuItem } from 'primeng/api';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <p-commandmenu [model]="commands" [filter]="fuzzyFilter" placeholder="Search for commands..." class="mx-auto">
            <ng-template #emptyMessage>No results found.</ng-template>
            <ng-template #footer>
                <div class="flex items-center justify-end gap-3 w-full">
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-up"></svg>
                        </kbd>
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                            <svg data-p-icon="arrow-down"></svg>
                        </kbd>
                        Navigate
                    </span>
                    <span class="flex items-center gap-1 text-surface-500 text-xs">
                        <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700"> &#x21B5; </kbd>
                        Select
                    </span>
                </div>
            </ng-template>
        </p-commandmenu>
    `,
    standalone: true,
    imports: [CommandMenuModule, ArrowUp, ArrowDown]
})
export class CommandMenuFilterDemo {
    commands: MenuItem[] = [
        {
            label: 'Recents',
            items: [{ label: 'Check For Updates', keywords: ['check', 'updates'] }, { label: 'Open Settings' }, { label: 'Search Files' }, { label: 'Open Terminal' }, { label: 'View History', keywords: ['history', 'recent'] }, { label: 'Open Chat' }]
        },
        {
            label: 'Files',
            items: [
                { label: 'New File' },
                { label: 'New Folder' },
                { label: 'Save All' },
                { label: 'Change Theme' },
                { label: 'Run Task' },
                { label: 'Stop Task' },
                { label: 'Export Project' },
                { label: 'Import Project' },
                { label: 'Delete File' },
                { label: 'Duplicate File' }
            ]
        },
        {
            label: 'Source',
            items: [{ label: 'Git: Commit' }, { label: 'Git: Push' }, { label: 'Git: Pull' }, { label: 'Switch Account' }, { label: 'Open Documentation' }, { label: 'Git: Sync' }, { label: 'Git: Create Branch' }, { label: 'Git: Create Tag' }]
        },
        {
            label: 'Editor',
            items: [{ label: 'Align Left' }, { label: 'Align Center' }, { label: 'Align Right' }, { label: 'Toggle Bold' }, { label: 'Toggle Italic' }, { label: 'Insert Link' }, { label: 'Insert Image' }, { label: 'Insert List' }]
        },
        {
            label: 'Navigation',
            items: [{ label: 'Go to Home' }, { label: 'Go Back' }, { label: 'Go Forward' }, { label: 'Open Explorer' }, { label: 'View Bookmarks' }, { label: 'Open Minimap' }]
        },
        {
            label: 'View',
            items: [{ label: 'Toggle Preview' }, { label: 'Maximize Window' }, { label: 'Minimize Window' }, { label: 'Grid View' }, { label: 'List View' }, { label: 'Light Mode' }, { label: 'Dark Mode' }]
        },
        {
            label: 'Tools',
            items: [{ label: 'Open Calculator' }, { label: 'Open Calendar' }, { label: 'Open Timer' }, { label: 'View Analytics' }, { label: 'View Trends' }, { label: 'Open Database' }]
        }
    ];

    fuzzyFilter(item: MenuItem, search: string): number {
        if (!search) return 1;

        const value = (item.label ?? '').toLowerCase();

        search = search.toLowerCase();

        let tIndex = 0;
        let sIndex = 0;
        let score = 0;

        while (tIndex < value.length && sIndex < search.length) {
            if (value[tIndex] === search[sIndex]) {
                score += 1;
                sIndex++;
            }

            tIndex++;
        }

        return sIndex === search.length ? score / value.length : 0;
    }
}
```

## With Dialog

CommandMenu can be used inside a Dialog to create a command palette experience. Press Ctrl+K (or Cmd+K on Mac) to open.

```typescript
import { Component } from '@angular/core';
import { Dialog, DialogModule } from 'primeng/dialog';
import { CommandMenuModule } from 'primeng/commandmenu';
import { MenuItem } from 'primeng/api';
import { ArrowUp } from '@primeicons/angular/arrow-up';
import { ArrowDown } from '@primeicons/angular/arrow-down';

@Component({
    template: `
        <div class="flex items-center justify-center py-8">
            <span class="cursor-pointer" (click)="visible = true">
                Press
                <kbd class="bg-surface-100 dark:bg-surface-950 px-2 py-1 rounded-md border border-surface-200 dark:border-surface-700/50 text-sm ml-2"> CTRL/⌘ + K </kbd>
            </span>
        </div>
        <p-dialog [(visible)]="visible" [modal]="true" [showHeader]="false" styleClass="overflow-hidden max-w-md w-full" [dismissableMask]="true" [contentStyle]="{ padding: '0' }" (onShow)="onDialogShow()">
            <p-commandmenu [model]="commands" [(search)]="searchValue" placeholder="Search for commands..." class="w-full rounded-none border-none" (onSelect)="visible = false">
                <ng-template #emptyMessage let-search>
                    No results found for <span class="text-surface-900 dark:text-surface-0">"{{ search }}"</span>
                </ng-template>
                <ng-template #footer>
                    <div class="flex items-center justify-end gap-3 w-full">
                        <span class="flex items-center gap-1 text-surface-500 text-xs">
                            <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                                <svg data-p-icon="arrow-up"></svg>
                            </kbd>
                            <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700">
                                <svg data-p-icon="arrow-down"></svg>
                            </kbd>
                            Navigate
                        </span>
                        <span class="flex items-center gap-1 text-surface-500 text-xs">
                            <kbd class="bg-surface-100 dark:bg-surface-800 size-5 inline-flex items-center justify-center rounded border border-surface-200 dark:border-surface-700"> &#x21B5; </kbd>
                            Select
                        </span>
                    </div>
                </ng-template>
            </p-commandmenu>
        </p-dialog>
    `,
    standalone: true,
    imports: [DialogModule, CommandMenuModule, ArrowUp, ArrowDown]
})
export class CommandMenuWithDialogDemo {
    visible: boolean = false;

    searchValue: string = '';

    commands: MenuItem[] = [
        {
            label: 'Recents',
            items: [{ label: 'Check For Updates', keywords: ['check', 'updates'] }, { label: 'Open Settings' }, { label: 'Search Files' }, { label: 'Open Terminal' }, { label: 'View History', keywords: ['history', 'recent'] }, { label: 'Open Chat' }]
        },
        {
            label: 'Files',
            items: [
                { label: 'New File' },
                { label: 'New Folder' },
                { label: 'Save All' },
                { label: 'Change Theme' },
                { label: 'Run Task' },
                { label: 'Stop Task' },
                { label: 'Export Project' },
                { label: 'Import Project' },
                { label: 'Delete File' },
                { label: 'Duplicate File' }
            ]
        },
        {
            label: 'Source',
            items: [{ label: 'Git: Commit' }, { label: 'Git: Push' }, { label: 'Git: Pull' }, { label: 'Switch Account' }, { label: 'Open Documentation' }, { label: 'Git: Sync' }, { label: 'Git: Create Branch' }, { label: 'Git: Create Tag' }]
        },
        {
            label: 'Editor',
            items: [{ label: 'Align Left' }, { label: 'Align Center' }, { label: 'Align Right' }, { label: 'Toggle Bold' }, { label: 'Toggle Italic' }, { label: 'Insert Link' }, { label: 'Insert Image' }, { label: 'Insert List' }]
        },
        {
            label: 'Navigation',
            items: [{ label: 'Go to Home' }, { label: 'Go Back' }, { label: 'Go Forward' }, { label: 'Open Explorer' }, { label: 'View Bookmarks' }, { label: 'Open Minimap' }]
        },
        {
            label: 'View',
            items: [{ label: 'Toggle Preview' }, { label: 'Maximize Window' }, { label: 'Minimize Window' }, { label: 'Grid View' }, { label: 'List View' }, { label: 'Light Mode' }, { label: 'Dark Mode' }]
        },
        {
            label: 'Tools',
            items: [{ label: 'Open Calculator' }, { label: 'Open Calendar' }, { label: 'Open Timer' }, { label: 'View Analytics' }, { label: 'View Trends' }, { label: 'Open Database' }]
        }
    ];

    handleKeyboardEvent(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
            event.preventDefault();
            event.stopPropagation();
            this.visible = !this.visible;
        }
    }

    onDialogShow() {
        this.commandmenu.focusInput();
    }
}
```

## Command Menu

CommandMenu is a search-driven command palette component.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, CommandMenuPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| model | MenuItem[] \| undefined | - | An array of menu items to display. |
| search | string | - | Two-way bound search value. |
| filter | ((item: MenuItem, search: string) => number) \| undefined | - | Custom filter function. Receives (item, search) and returns a number (0 = no match). |
| emptyMessage | string \| undefined | - | Text to display when there are no results. Defaults to global value in i18n translation configuration. |
| placeholder | string \| undefined | - | Placeholder text for the search input. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| ariaLabelledBy | string \| undefined | - | Identifies the element that labels the input for accessibility. |
| menuStyle | { [klass: string]: any } \| undefined | - | Inline style of the menu. |
| menuStyleClass | string \| undefined | - | Style class of the menu. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onSearchChange | event: CommandMenuSearchChangeEvent | Callback to invoke when the search value changes. |
| onSelect | event: CommandMenuItemSelectEvent | Callback to invoke when a menu item is selected. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<CommandMenuItemTemplateContext<any>> \| undefined | Custom item template. |
| header | TemplateRef<CommandMenuHeaderTemplateContext> \| undefined | Custom header template. |
| footer | TemplateRef<CommandMenuFooterTemplateContext> \| undefined | Custom footer template. |
| emptymessage | TemplateRef<CommandMenuEmptyMessageTemplateContext> \| undefined | Custom empty message template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |
| pcMenu | MenuPassThrough | Used to pass attributes to the Menu component. |
| emptyMessage | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the empty message's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-commandmenu | Class name of the root element |
| p-commandmenu-header | Class name of the header element |
| p-commandmenu-input | Class name of the input element |
| p-commandmenu-empty-message | Class name of the empty message element |
| p-commandmenu-footer | Class name of the footer element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| commandmenu.background | --p-commandmenu-background | Background of root |
| commandmenu.border.color | --p-commandmenu-border-color | Border color of root |
| commandmenu.border.radius | --p-commandmenu-border-radius | Border radius of root |
| commandmenu.height | --p-commandmenu-height | Height of root |
| commandmenu.header.padding | --p-commandmenu-header-padding | Padding of header |
| commandmenu.header.background | --p-commandmenu-header-background | Background of header |
| commandmenu.header.border.color | --p-commandmenu-header-border-color | Border color of header |
| commandmenu.input.padding | --p-commandmenu-input-padding | Padding of input |
| commandmenu.input.font.size | --p-commandmenu-input-font-size | Font size of input |
| commandmenu.input.font.weight | --p-commandmenu-input-font-weight | Font weight of input |
| commandmenu.input.color | --p-commandmenu-input-color | Color of input |
| commandmenu.input.placeholder.color | --p-commandmenu-input-placeholder-color | Placeholder color of input |
| commandmenu.list.padding | --p-commandmenu-list-padding | Padding of list |
| commandmenu.empty.padding | --p-commandmenu-empty-padding | Padding of empty |
| commandmenu.empty.color | --p-commandmenu-empty-color | Color of empty |
| commandmenu.footer.padding | --p-commandmenu-footer-padding | Padding of footer |
| commandmenu.footer.background | --p-commandmenu-footer-background | Background of footer |
| commandmenu.footer.border.color | --p-commandmenu-footer-border-color | Border color of footer |
