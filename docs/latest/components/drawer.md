---
title: "Angular Drawer Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/drawer.md"
description: "Drawer is a panel component displayed as an overlay at the edges of the screen."
---

# Angular Drawer Component

Drawer is a panel component displayed as an overlay at the edges of the screen.

## Accessibility

Screen Reader Drawer component uses complementary role by default, since any attribute is passed to the root element aria role can be changed depending on your use case and additional attributes like aria-labelledby can be added. In addition aria-modal is added since focus is kept within the drawer when opened. It is recommended to use a trigger component that can be accessed with keyboard such as a button, if not adding tabIndex would be necessary. Trigger element also requires aria-expanded and aria-controls to be handled explicitly. Overlay Keyboard Support Key Function tab Moves focus to the next the focusable element within the drawer. shift + tab Moves focus to the previous the focusable element within the drawer. escape Closes the dialog if closeOnEscape is true. Close Button Keyboard Support Key Function enter Closes the drawer. space Closes the drawer.

## Basic

A slide-in panel from the edge of the screen for contextual content.

```typescript
import { Component } from '@angular/core';
import { Drawer, DrawerModule } from 'primeng/drawer';
import { ButtonModule } from 'primeng/button';
import { Bars } from '@primeicons/angular/bars';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer [(visible)]="visible" header="Basic Drawer" styleClass="w-full! md:w-80!">
                <p>A slide-in panel from the edge of the screen for contextual content.</p>
            </p-drawer>
            <button type="button" pButton iconOnly (click)="visible = true">
                <svg data-p-icon="bars" />
            </button>
        </div>
    `,
    standalone: true,
    imports: [DrawerModule, ButtonModule, Bars]
})
export class DrawerBasicDemo {
    visible: boolean = false;
}
```

## Full Screen

The full screen mode is enabled when fullScreen property is set as true .

```typescript
import { Component } from '@angular/core';
import { Drawer, DrawerModule } from 'primeng/drawer';
import { ButtonModule } from 'primeng/button';
import { WindowMaximize } from '@primeicons/angular/window-maximize';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer [(visible)]="visible" header="Full Screen Drawer" [fullScreen]="true">
                <p>The full screen mode is enabled when position property is set as full.</p>
            </p-drawer>
            <button type="button" pButton iconOnly (click)="visible = true">
                <svg data-p-icon="window-maximize" />
            </button>
        </div>
    `,
    standalone: true,
    imports: [DrawerModule, ButtonModule, WindowMaximize]
})
export class DrawerFullscreenDemo {
    visible: boolean = false;
}
```

## Headless

Headless mode allows you to customize the entire user interface instead of the default elements.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { Drawer, DrawerModule } from 'primeng/drawer';
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { StyleClassModule } from 'primeng/styleclass';
import { Table } from 'primeng/table';
import { Times } from '@primeicons/angular/times';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { Home } from '@primeicons/angular/home';
import { Bookmark } from '@primeicons/angular/bookmark';
import { ChartLine } from '@primeicons/angular/chart-line';
import { Table } from '@primeicons/angular/table';
import { Search } from '@primeicons/angular/search';
import { Users } from '@primeicons/angular/users';
import { Comments } from '@primeicons/angular/comments';
import { Calendar } from '@primeicons/angular/calendar';
import { Cog } from '@primeicons/angular/cog';
import { Folder } from '@primeicons/angular/folder';
import { ChartBar } from '@primeicons/angular/chart-bar';
import { Bars } from '@primeicons/angular/bars';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer #drawerRef [(visible)]="visible">
                <ng-template #headless>
                    <div class="flex flex-col h-full">
                        <div class="flex items-center justify-between px-5 pt-3 shrink-0">
                            <span class="inline-flex items-center gap-2">
                                <svg width="31" height="33" viewBox="0 0 31 33" fill="none" xmlns="http://www.w3.org/2000/svg" class="block mx-auto">
                                    <path d="M15.1934 0V0V0L0.0391235 5.38288L2.35052 25.3417L15.1934 32.427V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1934 0Z" fill="var(--p-primary-color)" />
                                    <mask id="mask0_1_52" style="mask-type:luminance" maskUnits="userSpaceOnUse" x="0" y="0" width="31" height="33">
                                        <path d="M15.1934 0V0V0L0.0391235 5.38288L2.35052 25.3417L15.1934 32.427V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1934 0Z" fill="var(--ground-background)" />
                                    </mask>
                                    <g mask="url(#mask0_1_52)">
                                        <path fill-rule="evenodd" clip-rule="evenodd" d="M15.1935 0V3.5994V3.58318V20.0075V20.0075V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1935 0Z" fill="var(--p-primary-color)" />
                                    </g>
                                    <path d="M19.6399 15.3776L18.1861 15.0547L19.3169 16.6695V21.6755L23.1938 18.4458V12.9554L21.4169 13.6013L19.6399 15.3776Z" fill="var(--ground-background)" />
                                    <path d="M10.5936 15.3776L12.0474 15.0547L10.9166 16.6695V21.6755L7.03966 18.4458V12.9554L8.81661 13.6013L10.5936 15.3776Z" fill="var(--ground-background)" />
                                    <path
                                        fill-rule="evenodd"
                                        clip-rule="evenodd"
                                        d="M11.3853 16.9726L12.6739 15.0309L13.4793 15.5163H16.7008L17.5061 15.0309L18.7947 16.9726V24.254L17.8283 25.7103L16.7008 26.843H13.4793L12.3518 25.7103L11.3853 24.254V16.9726Z"
                                        fill="var(--ground-background)"
                                    />
                                    <path d="M19.3168 24.7437L21.4168 22.6444V20.5451L19.3168 22.3214V24.7437Z" fill="var(--ground-background)" />
                                    <path d="M10.9166 24.7437L8.81662 22.6444V20.5451L10.9166 22.3214V24.7437Z" fill="var(--ground-background)" />
                                    <path
                                        fill-rule="evenodd"
                                        clip-rule="evenodd"
                                        d="M13.0167 5.68861L11.7244 8.7568L13.8244 14.8932H14.7936V5.68861H13.0167ZM15.4397 5.68861V14.8932H16.5706L18.5091 8.7568L17.2167 5.68861H15.4397Z"
                                        fill="var(--ground-background)"
                                    />
                                    <path d="M13.8244 14.8932L6.87813 12.3094L5.90888 8.27235L11.8859 8.7568L13.9859 14.8932H13.8244Z" fill="var(--ground-background)" />
                                    <path d="M16.5706 14.8932L23.5169 12.3094L24.4861 8.27235L18.3476 8.7568L16.4091 14.8932H16.5706Z" fill="var(--ground-background)" />
                                    <path d="M18.8321 8.27235L22.2245 7.94938L19.9629 5.68861H17.7013L18.8321 8.27235Z" fill="var(--ground-background)" />
                                    <path d="M11.4013 8.27235L8.00893 7.94938L10.2705 5.68861H12.5321L11.4013 8.27235Z" fill="var(--ground-background)" />
                                </svg>
                                <span class="font-semibold text-xl text-primary">Your Logo</span>
                            </span>
                            <span>
                                <button type="button" pButton iconOnly (click)="closeCallback($event)" rounded="true" outlined="true" class="h-8 w-8"><svg data-p-icon="times" /></button>
                            </span>
                        </div>
                        <div class="overflow-y-auto">
                            <ul class="list-none p-3 m-0">
                                <li>
                                    <div
                                        pRipple
                                        pStyleClass="@next"
                                        enterFromClass="hidden"
                                        enterActiveClass="animate-slidedown"
                                        leaveToClass="hidden"
                                        leaveActiveClass="animate-slideup"
                                        class="p-3 flex items-center justify-between text-surface-600 dark:text-surface-200 cursor-pointer p-ripple"
                                    >
                                        <span class="font-medium text-sm">FAVORITES</span>
                                        <svg data-p-icon="chevron-down" />
                                    </div>
                                    <ul class="list-none p-0 m-0 overflow-hidden">
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="home" class="mr-2" />
                                                <span class="font-medium text-sm">Dashboard</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="bookmark" class="mr-2" />
                                                <span class="font-medium text-sm">Bookmarks</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                pRipple
                                                pStyleClass="@next"
                                                enterFromClass="hidden"
                                                enterActiveClass="animate-slidedown"
                                                leaveToClass="hidden"
                                                leaveActiveClass="animate-slideup"
                                                class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple"
                                            >
                                                <svg data-p-icon="chart-line" class="mr-2" />
                                                <span class="font-medium text-sm">Reports</span>
                                                <svg data-p-icon="chevron-down" class="ml-auto" />
                                            </a>
                                            <ul class="list-none py-0 pl-4 pr-0 m-0 hidden overflow-y-hidden transition-all duration-400 ease-in-out">
                                                <li>
                                                    <a
                                                        pRipple
                                                        pStyleClass="@next"
                                                        enterFromClass="hidden"
                                                        enterActiveClass="animate-slidedown"
                                                        leaveToClass="hidden"
                                                        leaveActiveClass="animate-slideup"
                                                        class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple"
                                                    >
                                                        <svg data-p-icon="chart-line" class="mr-2" />
                                                        <span class="font-medium text-sm">Revenue</span>
                                                        <svg data-p-icon="chevron-down" class="ml-auto" />
                                                    </a>
                                                    <ul class="list-none py-0 pl-4 pr-0 m-0 hidden overflow-y-hidden transition-all duration-400 ease-in-out">
                                                        <li>
                                                            <a
                                                                pRipple
                                                                class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple"
                                                            >
                                                                <svg data-p-icon="table" class="mr-2" />
                                                                <span class="font-medium text-sm">View</span>
                                                            </a>
                                                        </li>
                                                        <li>
                                                            <a
                                                                pRipple
                                                                class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple"
                                                            >
                                                                <svg data-p-icon="search" class="mr-2" />
                                                                <span class="font-medium text-sm">Search</span>
                                                            </a>
                                                        </li>
                                                    </ul>
                                                </li>
                                                <li>
                                                    <a
                                                        pRipple
                                                        class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple"
                                                    >
                                                        <svg data-p-icon="chart-line" class="mr-2" />
                                                        <span class="font-medium text-sm">Expenses</span>
                                                    </a>
                                                </li>
                                            </ul>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="users" class="mr-2" />
                                                <span class="font-medium text-sm">Team</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="comments" class="mr-2" />
                                                <span class="font-medium text-sm">Messages</span>
                                                <span class="inline-flex items-center justify-center ml-auto bg-primary text-primary-contrast rounded-full text-sm" style="min-width: 1.4rem; height: 1.4rem">3</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="calendar" class="mr-2" />
                                                <span class="font-medium text-sm">Calendar</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="cog" class="mr-2" />
                                                <span class="font-medium text-sm">Settings</span>
                                            </a>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                            <ul class="list-none p-3 m-0">
                                <li>
                                    <div
                                        pRipple
                                        pStyleClass="@next"
                                        enterFromClass="hidden"
                                        enterActiveClass="animate-slidedown"
                                        leaveToClass="hidden"
                                        leaveActiveClass="animate-slideup"
                                        class="p-3 flex items-center justify-between text-surface-600 dark:text-surface-200 cursor-pointer p-ripple"
                                    >
                                        <span class="font-medium text-sm">APPLICATION</span>
                                        <svg data-p-icon="chevron-down" />
                                    </div>
                                    <ul class="list-none p-0 m-0 overflow-hidden">
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="folder" class="mr-2" />
                                                <span class="font-medium text-sm">Projects</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="chart-bar" class="mr-2" />
                                                <span class="font-medium text-sm">Performance</span>
                                            </a>
                                        </li>
                                        <li>
                                            <a pRipple class="flex items-center cursor-pointer p-3 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                                <svg data-p-icon="cog" class="mr-2" />
                                                <span class="font-medium text-sm">Settings</span>
                                            </a>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                        </div>
                        <div class="mt-auto">
                            <hr class="mb-3 mx-3 border-t border-0 border-surface" />
                            <a pRipple class="m-3 flex items-center cursor-pointer p-3 gap-2 rounded-border text-surface-700 dark:text-surface-100 hover:bg-surface-100 dark:hover:bg-surface-700 duration-150 transition-colors p-ripple">
                                <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape="circle" />
                                <span class="font-bold text-sm">Amy Elsner</span>
                            </a>
                        </div>
                    </div>
                </ng-template>
            </p-drawer>
            <button type="button" pButton iconOnly (click)="visible = true"><svg data-p-icon="bars" /></button>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, DrawerModule, ButtonModule, RippleModule, StyleClassModule, Times, ChevronDown, Home, Bookmark, ChartLine, Table, Search, Users, Comments, Calendar, Cog, Folder, ChartBar, Bars]
})
export class DrawerHeadlessDemo {
    visible: boolean = false;

    closeCallback(e): void {
        this.drawerRef.close(e);
    }
}
```

## Position

The position of the drawer can be customized with the position property. The available values are left , right , top and bottom .

```typescript
import { Component } from '@angular/core';
import { Drawer, DrawerModule } from 'primeng/drawer';
import { ButtonModule } from 'primeng/button';
import { ArrowRight } from '@primeicons/angular/arrow-right';
import { ArrowLeft } from '@primeicons/angular/arrow-left';
import { ArrowDown } from '@primeicons/angular/arrow-down';
import { ArrowUp } from '@primeicons/angular/arrow-up';

@Component({
    template: `
        <p-drawer [(visible)]="visibleLeft" header="Left Drawer" position="left" styleClass="w-full! md:w-80!">
            <p>The position of the drawer can be customized with the position property. The available values are left, right, top and bottom.</p>
        </p-drawer>
        <p-drawer [(visible)]="visibleRight" header="Right Drawer" position="right" styleClass="w-full! md:w-80!">
            <p>The position of the drawer can be customized with the position property. The available values are left, right, top and bottom.</p>
        </p-drawer>
        <p-drawer [(visible)]="visibleTop" header="Top Drawer" position="top" [style]="{ height: 'auto' }">
            <p>The position of the drawer can be customized with the position property. The available values are left, right, top and bottom.</p>
        </p-drawer>
        <p-drawer [(visible)]="visibleBottom" header="Bottom Drawer" position="bottom" [style]="{ height: 'auto' }">
            <p>The position of the drawer can be customized with the position property. The available values are left, right, top and bottom.</p>
        </p-drawer>
        <div class="flex gap-2 justify-center">
            <button type="button" pButton iconOnly (click)="visibleLeft = true">
                <svg data-p-icon="arrow-right" />
            </button>
            <button type="button" pButton iconOnly (click)="visibleRight = true">
                <svg data-p-icon="arrow-left" />
            </button>
            <button type="button" pButton iconOnly (click)="visibleTop = true">
                <svg data-p-icon="arrow-down" />
            </button>
            <button type="button" pButton iconOnly (click)="visibleBottom = true">
                <svg data-p-icon="arrow-up" />
            </button>
        </div>
    `,
    standalone: true,
    imports: [DrawerModule, ButtonModule, ArrowRight, ArrowLeft, ArrowDown, ArrowUp]
})
export class DrawerPositionDemo {
    visibleLeft: boolean = false;

    visibleRight: boolean = false;

    visibleTop: boolean = false;

    visibleBottom: boolean = false;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { BadgeModule } from 'primeng/badge';
import { DrawerModule } from 'primeng/drawer';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { MenuModule } from 'primeng/menu';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { MenuItem } from 'primeng/api';
import { Search } from '@primeicons/angular/search';
import { Cog } from '@primeicons/angular/cog';
import { Bars } from '@primeicons/angular/bars';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer [(visible)]="visible" header="Drawer Title" styleClass="w-full! md:w-80!" [pt]="{ content: { class: 'p-0!' } }">
                <ng-template #content>
                    <div class="flex flex-col h-full">
                        <div class="px-4 pb-4">
                            <p-iconfield>
                                <p-inputicon>
                                    <svg data-p-icon="search" />
                                </p-inputicon>
                                <input pInputText [(ngModel)]="searchValue" placeholder="Search..." class="w-full" />
                            </p-iconfield>
                        </div>
                        <div class="flex-1 overflow-y-auto px-4">
                            <p-menu [model]="menuItems" styleClass="w-full border-0! bg-transparent! p-0!" [pt]="{ list: { class: 'gap-0!' } }">
                                <ng-template #submenuheader let-item>
                                    <span [class]="'text-surface-500 dark:text-surface-500 font-semibold text-xs tracking-wider py-1 block' + (item.id !== 'main' ? ' mt-4' : '')">{{ item.label }}</span>
                                </ng-template>
                                <ng-template #item let-item>
                                    <a class="flex items-center gap-2 text-sm px-2.5 py-1 w-full cursor-pointer">
                                        @if (item.state?.color) {
                                            <span [class]="item.state.color + ' w-2 h-2 rounded-full shrink-0'"></span>
                                            <span class="truncate">{{ item.label }}</span>
                                            <span class="text-xs text-surface-400 ml-auto shrink-0">{{ item.state.time }}</span>
                                        } @else {
                                            <i [class]="item.icon + ' text-base' + (item.state?.active ? ' text-primary' : '')"></i>
                                            <span [class]="item.state?.active ? 'text-primary font-semibold' : ''">{{ item.label }}</span>
                                            @if (item.badge) {
                                                <p-badge [value]="item.badge" class="ml-auto" />
                                            }
                                        }
                                    </a>
                                </ng-template>
                            </p-menu>
                        </div>
                    </div>
                </ng-template>
                <ng-template #footer>
                    <div class="flex items-center gap-3 p-2 rounded-lg">
                        <p-avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
                        <div class="flex flex-col flex-1 min-w-0">
                            <span class="font-semibold text-sm truncate">Sarah Anderson</span>
                            <span class="text-xs text-surface-500 truncate">sarah&#64;primetek.com</span>
                        </div>
                        <button type="button" pButton iconOnly variant="text" severity="secondary" rounded>
                            <svg data-p-icon="cog" />
                        </button>
                    </div>
                </ng-template>
            </p-drawer>
            <button type="button" pButton iconOnly (click)="visible = true">
                <svg data-p-icon="bars" />
            </button>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, BadgeModule, DrawerModule, IconFieldModule, InputIconModule, MenuModule, ButtonModule, InputTextModule, FormsModule, Search, Cog, Bars]
})
export class DrawerPreviewDemo {
    visible: boolean = false;

    searchValue: string = '';

    menuItems: MenuItem[] = [
        {
            id: 'main',
            label: 'MAIN',
            items: [
                { label: 'Dashboard', icon: 'pi pi-objects-column', badge: '3', state: { active: true } },
                { label: 'Inbox', icon: 'pi pi-inbox', badge: '12' },
                { label: 'Calendar', icon: 'pi pi-calendar' },
                { label: 'Analytics', icon: 'pi pi-chart-bar' }
            ]
        },
        {
            label: 'WORKSPACE',
            items: [
                { label: 'Projects', icon: 'pi pi-folder', badge: '5' },
                { label: 'Team', icon: 'pi pi-users' },
                { label: 'Documents', icon: 'pi pi-file' },
                { label: 'Tasks', icon: 'pi pi-clipboard' }
            ]
        },
        {
            label: 'RECENT',
            items: [
                { label: 'Website Redesign', state: { color: 'bg-blue-500', time: '2m ago' } },
                { label: 'API Integration', state: { color: 'bg-green-500', time: '1h ago' } },
                { label: 'Mobile App v2', state: { color: 'bg-orange-500', time: '3h ago' } }
            ]
        }
    ];
}
```

## Responsive

The responsive mode can be enabled by adding styleClass with responsive utilities.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { Drawer, DrawerModule } from 'primeng/drawer';
import { PasswordModule } from 'primeng/password';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer [(visible)]="visible" header="Responsive Drawer" styleClass="w-full! sm:w-96! md:w-md! lg:w-120!">
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label for="email" class="font-medium text-sm">Email</label>
                        <input pInputText id="email" placeholder="Enter your email" class="w-full" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label for="password" class="font-medium text-sm">Password</label>
                        <p-password id="password" placeholder="Enter your password" [feedback]="false" styleClass="w-full" inputStyleClass="w-full" />
                    </div>
                    <div class="flex items-center gap-2">
                        <p-checkbox [(ngModel)]="remember" [binary]="true" inputId="remember" />
                        <label for="remember" class="text-sm">Remember me</label>
                    </div>
                    <button type="button" pButton class="w-full">Sign In</button>
                </div>
            </p-drawer>
            <button type="button" pButton (click)="visible = true">Log in</button>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, DrawerModule, PasswordModule, ButtonModule, InputTextModule, FormsModule]
})
export class DrawerResponsiveDemo {
    visible: boolean = false;

    remember: boolean = false;
}
```

## Template

Drawer is customizable by header , content , footer templates.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { DrawerModule } from 'primeng/drawer';
import { ButtonModule } from 'primeng/button';
import { User } from '@primeicons/angular/user';
import { SignOut } from '@primeicons/angular/sign-out';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <div class="flex justify-center">
            <p-drawer [(visible)]="visible" [closable]="false">
                <ng-template #header>
                    <div class="flex items-center gap-2">
                        <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape="circle" />
                        <span class="font-bold">Amy Elsner</span>
                    </div>
                </ng-template>
                <p class="text-sm">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                    consequat.
                </p>
                <ng-template #footer>
                    <div class="flex items-center gap-2">
                        <button type="button" pButton class="w-full" outlined>
                            <svg data-p-icon="user" />
                            Account
                        </button>
                        <button type="button" pButton class="w-full" severity="danger" text>
                            <svg data-p-icon="sign-out" />
                            Logout
                        </button>
                    </div>
                </ng-template>
            </p-drawer>
            <button type="button" pButton iconOnly (click)="visible = true">
                <svg data-p-icon="plus" />
            </button>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, DrawerModule, ButtonModule, User, SignOut, Plus]
})
export class DrawerTemplateDemo {
    visible: boolean = false;
}
```

## Drawer

Sidebar is a panel component displayed as an overlay at the edges of the screen.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, DrawerPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'self' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| blockScroll | boolean | - | Whether to block scrolling of the document when drawer is active. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| styleClass | string \| undefined | - | Style class of the component. |
| ariaCloseLabel | string \| undefined | - | Aria label of the close icon. |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| modal | boolean | - | Whether an overlay mask is displayed behind the drawer. |
| closeButtonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the close button. |
| dismissible | boolean | - | Whether to dismiss drawer on click of the mask. |
| showCloseIcon | boolean | - | Whether to display the close icon. **(Deprecated)** |
| closeOnEscape | boolean | - | Specifies if pressing escape key should hide the drawer. |
| visible | boolean | false | The visible property is an input that determines the visibility of the component. |
| position | "left" \| "right" \| "bottom" \| "top" \| "full" | 'left' | Specifies the position of the drawer, valid values are "left", "right", "bottom" and "top". |
| fullScreen | boolean | false | Adds a close icon to the header to hide the dialog. |
| header | string \| undefined | - | Title content of the dialog. |
| maskStyle | { [klass: string]: any } \| null \| undefined | - | Style of the mask. |
| closable | boolean | true | Whether to display close button. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onShow | value: any | Callback to invoke when dialog is shown. |
| onHide | value: any | Callback to invoke when dialog is hidden. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<void> \| undefined | Custom header template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| content | TemplateRef<void> \| undefined | Custom content template. |
| closeicon | TemplateRef<void> \| undefined | Custom close icon template. |
| headless | TemplateRef<void> \| undefined | Custom headless template to replace the entire drawer content. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| title | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the title's DOM element. |
| pcCloseButton | ButtonPassThrough | Used to pass attributes to the close Button component. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-drawer-mask | Class name of the mask element |
| p-drawer | Class name of the root element |
| p-drawer-header | Class name of the header element |
| p-drawer-title | Class name of the title element |
| p-drawer-close-button | Class name of the close button element |
| p-drawer-content | Class name of the content element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| drawer.background | --p-drawer-background | Background of root |
| drawer.border.color | --p-drawer-border-color | Border color of root |
| drawer.color | --p-drawer-color | Color of root |
| drawer.shadow | --p-drawer-shadow | Shadow of root |
| drawer.header.padding | --p-drawer-header-padding | Padding of header |
| drawer.title.font.size | --p-drawer-title-font-size | Font size of title |
| drawer.title.font.weight | --p-drawer-title-font-weight | Font weight of title |
| drawer.content.padding | --p-drawer-content-padding | Padding of content |
| drawer.footer.padding | --p-drawer-footer-padding | Padding of footer |
