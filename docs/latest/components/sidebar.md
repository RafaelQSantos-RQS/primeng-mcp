---
title: "Angular Sidebar Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/sidebar.md"
description: "Sidebar is a compound navigation panel with collapsible icon mode, offcanvas mode, variants and optional overlay."
---

# Angular Sidebar Component

Sidebar is a compound navigation panel with collapsible icon mode, offcanvas mode, variants and optional overlay.

## Accessibility

Screen Reader The sidebar is a navigation landmark. Wrap menus in a &lt;nav&gt; when appropriate and provide an aria-label to describe the region. Menu items render as role="listitem" inside role="list" containers. The pSidebarTrigger directive exposes aria-controls / aria-expanded tied to the sidebar it toggles. When used as an overlay with dismissable , pressing Escape closes the sidebar.

## Chat Application

ChatGPT-style history sidebar with grouped sessions and hover-revealed delete action on each entry.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { Sparkles } from '@primeicons/angular/sparkles';
import { Search } from '@primeicons/angular/search';
import { PenToSquare } from '@primeicons/angular/pen-to-square';
import { Globe } from '@primeicons/angular/globe';
import { Trash } from '@primeicons/angular/trash';
import { Sidebar } from '@primeicons/angular/sidebar';

interface ChatEntry {
    title: string;
    active?: boolean;
}

interface ChatGroup {
    label: string;
    chats: ChatEntry[];
}

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && open()) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="chat-history" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [(open)]="open" width="16rem">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-full bg-surface-900 dark:bg-surface-0">
                                                <svg data-p-icon="sparkles" [size]="12" class="text-surface-0 dark:text-surface-900"></svg>
                                            </div>
                                            <span class="font-semibold text-sm">ChatGPT</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton>
                                            <svg data-p-icon="search"></svg>
                                            <span>Search</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton>
                                            <svg data-p-icon="pen-to-square"></svg>
                                            <span>New chat</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton>
                                            <svg data-p-icon="globe"></svg>
                                            <span>Browse web</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                @for (group of chatHistory; track group.label) {
                                    <p-sidebar-group>
                                        <p-sidebar-group-label>{{ group.label }}</p-sidebar-group-label>
                                        <p-sidebar-group-content>
                                            <p-sidebar-menu>
                                                @for (chat of group.chats; track chat.title) {
                                                    <p-sidebar-menu-item>
                                                        <button pSidebarMenuButton [isActive]="!!chat.active">
                                                            <span>{{ chat.title }}</span>
                                                        </button>
                                                        <button pSidebarMenuAction showOnHover><svg data-p-icon="trash"></svg></button>
                                                    </p-sidebar-menu-item>
                                                }
                                            </p-sidebar-menu>
                                        </p-sidebar-group-content>
                                    </p-sidebar-group>
                                }
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="chat-history" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, SidebarModule, ButtonModule, Sparkles, Search, PenToSquare, Globe, Trash, Sidebar]
})
export class SidebarChatDemo implements OnInit {
    isMobile = signal(false);

    open = signal(true);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    chatHistory: ChatGroup[] = [
        {
            label: 'Today',
            chats: [{ title: 'PrimeNG Sidebar component', active: true }, { title: 'How to use Angular signals' }, { title: 'Fix TypeScript generics issue' }]
        },
        {
            label: 'Yesterday',
            chats: [{ title: 'Design system architecture' }, { title: 'Angular 21 new features overview' }, { title: 'CSS light-dark() function usage' }]
        },
        {
            label: 'Previous 7 days',
            chats: [{ title: 'Build a dashboard layout' }, { title: 'Tailwind v4 migration guide' }, { title: 'Node.js API authentication' }, { title: 'Monorepo setup with pnpm' }, { title: 'Deploy Angular to Vercel' }]
        },
        {
            label: 'Previous 30 days',
            chats: [{ title: 'GraphQL schema design' }, { title: 'WebSocket real-time updates' }, { title: 'PostgreSQL query optimization' }, { title: 'Docker compose for dev env' }]
        }
    ];

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.open.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.open.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }
}
```

## Dual Sidebar

Two sidebars on opposite sides with independent triggers. The left is icon-collapsible navigation, the right is an offcanvas chat panel.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { Home } from '@primeicons/angular/home';
import { Inbox } from '@primeicons/angular/inbox';
import { Search } from '@primeicons/angular/search';
import { Bell } from '@primeicons/angular/bell';
import { ChartBar } from '@primeicons/angular/chart-bar';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { Users } from '@primeicons/angular/users';
import { Calendar } from '@primeicons/angular/calendar';
import { Cog } from '@primeicons/angular/cog';
import { Sidebar } from '@primeicons/angular/sidebar';
import { Comment } from '@primeicons/angular/comment';

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && (navOpen() || open())) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="nav" side="left" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [(open)]="navOpen" width="14rem">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-linear-to-br from-violet-500 to-indigo-600 text-white text-xs font-bold leading-none">A</div>
                                            <span class="font-semibold text-sm">Acme Inc</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Navigation</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton [isActive]="true">
                                                    <svg data-p-icon="home"></svg>
                                                    <span>Home</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="inbox"></svg>
                                                    <span>Inbox</span>
                                                </button>
                                                <p-sidebar-menu-badge>5</p-sidebar-menu-badge>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="search"></svg>
                                                    <span>Search</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="bell"></svg>
                                                    <span>Notifications</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Projects</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item collapsible [defaultOpen]="true">
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="chart-bar"></svg>
                                                    <span>Analytics</span>
                                                    <svg data-p-icon="chevron-down" class="ml-auto transition-transform duration-200"></svg>
                                                </button>
                                                <p-sidebar-menu-sub>
                                                    <p-sidebar-menu-sub-item>
                                                        <button pSidebarMenuSubButton [isActive]="true">Overview</button>
                                                    </p-sidebar-menu-sub-item>
                                                    <p-sidebar-menu-sub-item>
                                                        <button pSidebarMenuSubButton>Reports</button>
                                                    </p-sidebar-menu-sub-item>
                                                </p-sidebar-menu-sub>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="users"></svg>
                                                    <span>Team</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="calendar"></svg>
                                                    <span>Calendar</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton>
                                            <svg data-p-icon="cog"></svg>
                                            <span>Settings</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="nav" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                        <span class="text-sm font-medium flex-1">Dashboard</span>
                        <button pButton pSidebarTrigger target="ai" severity="secondary" text size="small">
                            <svg data-p-icon="comment"></svg>
                        </button>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
                <p-sidebar id="ai" side="right" collapsible="offcanvas" [overlay]="true" [(open)]="open" width="18rem">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <div class="flex items-center gap-2 px-1">
                                    <span class="font-semibold text-sm text-surface-500">Weekly metrics overview</span>
                                </div>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <div class="flex flex-col gap-4 px-3 py-2">
                                    <div class="rounded-xl bg-surface-100 dark:bg-surface-800 px-3 py-2.5 text-sm">Show me this week&apos;s metrics</div>
                                    <div class="text-sm leading-relaxed">
                                        <p>Here are your key metrics for this week:</p>
                                        <ul class="mt-2 space-y-1.5 text-muted-color">
                                            <li class="flex justify-between">
                                                <span>Page views</span>
                                                <span class="font-medium text-color">12,482</span>
                                            </li>
                                            <li class="flex justify-between">
                                                <span>New users</span>
                                                <span class="font-medium text-color">342</span>
                                            </li>
                                            <li class="flex justify-between">
                                                <span>Bounce rate</span>
                                                <span class="font-medium text-color">34%</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <div class="mx-1 rounded-xl border border-surface-200 dark:border-surface-700 px-3 py-2.5">
                                    <input type="text" placeholder="Reply to Claude..." class="w-full bg-transparent border-none outline-none text-sm" />
                                </div>
                            </p-sidebar-footer>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [SidebarModule, ButtonModule, Home, Inbox, Search, Bell, ChartBar, ChevronDown, Users, Calendar, Cog, Sidebar, Comment]
})
export class SidebarDualDemo implements OnInit {
    isMobile = signal(false);

    navOpen = signal(true);

    open = signal(false);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.navOpen.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.navOpen.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }
}
```

## With Menu

Full application chrome with workspace switcher, grouped navigation and user footer. Collapses to icon mode.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { MenuModule } from 'primeng/menu';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { MenuItem } from 'primeng/api';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { Check } from '@primeicons/angular/check';
import { Home } from '@primeicons/angular/home';
import { Inbox } from '@primeicons/angular/inbox';
import { Search } from '@primeicons/angular/search';
import { Users } from '@primeicons/angular/users';
import { Bell } from '@primeicons/angular/bell';
import { Sidebar } from '@primeicons/angular/sidebar';

interface Company {
    name: string;
    logo: string;
    color: string;
}

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && open()) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="menu-demo" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [(open)]="open">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!" (click)="companyMenu.toggle($event)">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md text-white text-xs font-bold leading-none bg-linear-to-br" [class]="activeCompany.color">
                                                {{ activeCompany.logo }}
                                            </div>
                                            <span class="font-semibold text-sm">{{ activeCompany.name }}</span>
                                            <svg data-p-icon="chevron-down" class="ml-auto"></svg>
                                        </button>
                                        <p-menu #companyMenu [model]="companyItems" [popup]="true" appendTo="body">
                                            <ng-template #item let-item>
                                                @if (item.separator) {
                                                    <hr class="border-surface-200 dark:border-surface-700 my-1" />
                                                } @else if (item.logo) {
                                                    <a class="flex items-center gap-2 px-3 py-2 cursor-pointer hover:bg-surface-100 dark:hover:bg-surface-800 rounded-md" (click)="item.command?.()">
                                                        <div class="flex size-5 shrink-0 items-center justify-center rounded-sm text-white text-[0.625rem] font-bold leading-none bg-linear-to-br" [class]="item.color">{{ item.logo }}</div>
                                                        <span class="text-sm">{{ item.label }}</span>
                                                        @if (item.active) {
                                                            <svg data-p-icon="check" class="ml-auto"></svg>
                                                        }
                                                    </a>
                                                } @else {
                                                    <a class="flex items-center gap-2 px-3 py-2 cursor-pointer hover:bg-surface-100 dark:hover:bg-surface-800 rounded-md" (click)="item.command?.()">
                                                        @if (item.icon) {
                                                            <span [class]="item.icon"></span>
                                                        }
                                                        <span class="text-sm">{{ item.label }}</span>
                                                    </a>
                                                }
                                            </ng-template>
                                        </p-menu>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Navigation</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton [isActive]="true">
                                                    <svg data-p-icon="home"></svg>
                                                    <span>Home</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="inbox"></svg>
                                                    <span>Inbox</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="search"></svg>
                                                    <span>Search</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="users"></svg>
                                                    <span>Team</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="bell"></svg>
                                                    <span>Notifications</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!" (click)="userMenu.toggle($event)">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                            <svg data-p-icon="chevron-down" class="ml-auto"></svg>
                                        </button>
                                        <p-menu #userMenu [model]="userItems" [popup]="true" appendTo="body" />
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                            <button pSidebarRail></button>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="menu-demo" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, MenuModule, SidebarModule, ButtonModule, ChevronDown, Check, Home, Inbox, Search, Users, Bell, Sidebar]
})
export class SidebarMenuDemo implements OnInit {
    isMobile = signal(false);

    open = signal(true);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    companies: Company[] = [
        { name: 'Acme Inc', logo: 'A', color: 'from-violet-500 to-indigo-600' },
        { name: 'Globex Corp', logo: 'G', color: 'from-emerald-500 to-teal-600' },
        { name: 'Initech', logo: 'I', color: 'from-orange-500 to-red-600' }
    ];

    activeCompany = this.companies[0];

    companyItems: MenuItem[] = [
        {
            label: 'Companies',
            items: [
                ...this.companies.map((c: any) => ({
                    label: c.name,
                    logo: c.logo,
                    color: c.color,
                    active: this.activeCompany.name === c.name,
                    command: () => {
                        this.activeCompany = c;
                        this.updateCompanyActive();
                    }
                })),
                { separator: true },
                { label: 'Add company', icon: 'pi pi-plus' }
            ]
        }
    ];

    userItems: MenuItem[] = [
        {
            label: 'john@acme.com',
            items: [{ label: 'Settings', icon: 'pi pi-cog' }, { label: 'Notifications', icon: 'pi pi-bell' }, { separator: true }, { label: 'Sign out', icon: 'pi pi-sign-out' }]
        }
    ];

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.open.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.open.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }

    updateCompanyActive(): void {
        const items = (this.companyItems[0] as any).items;

        items.forEach((item: any) => {
            if (item.logo) {
                item.active = this.activeCompany.name === item.label;
            }
        });
    }
}
```

## Multi Sidebar

Two stacked sidebars: a hover-to-open icon bar and a fixed secondary panel. Supabase-like IDE layout.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { SidebarModule } from 'primeng/sidebar';
import { Home } from '@primeicons/angular/home';
import { Database } from '@primeicons/angular/database';
import { Key } from '@primeicons/angular/key';
import { Server } from '@primeicons/angular/server';
import { Globe } from '@primeicons/angular/globe';
import { Code } from '@primeicons/angular/code';
import { Cog } from '@primeicons/angular/cog';
import { ExternalLink } from '@primeicons/angular/external-link';

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && (open() || secondaryOpen())) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <!-- Icon bar - always collapsed, opens on hover as overlay -->
                <p-sidebar id="iconbar" side="left" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="true" [openOnHover]="!isMobile()" width="12rem" [(open)]="open">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-emerald-600 text-white text-xs font-bold leading-none">A</div>
                                            <span class="font-semibold text-sm">Acme Inc</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton [isActive]="true">
                                                    <svg data-p-icon="home"></svg>
                                                    <span>Home</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="database"></svg>
                                                    <span>Database</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="key"></svg>
                                                    <span>Authentication</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="server"></svg>
                                                    <span>Edge Functions</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="globe"></svg>
                                                    <span>Storage</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="code"></svg>
                                                    <span>SQL Editor</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton>
                                            <svg data-p-icon="cog"></svg>
                                            <span>Settings</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <!-- Secondary sidebar - always open, not collapsible -->
                <p-sidebar id="secondary" side="left" width="14rem" [collapsible]="isMobile() ? 'offcanvas' : 'none'" [overlay]="isMobile()" [(open)]="secondaryOpen">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <div class="px-1"><span class="font-semibold text-sm">Settings</span></div>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Configuration</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton [isActive]="true"><span>General</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Compute and Disk</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Infrastructure</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Integrations</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>API Keys</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>JWT Keys</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Log Drains</span></button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Add-ons</span></button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Integrations</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <span>Data API</span>
                                                    <svg data-p-icon="external-link" class="ml-auto"></svg>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton><span>Vault</span></button>
                                                <p-sidebar-menu-badge>BETA</p-sidebar-menu-badge>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Billing</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <span>Subscription</span>
                                                    <svg data-p-icon="external-link" class="ml-auto"></svg>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <span>Usage</span>
                                                    <svg data-p-icon="external-link" class="ml-auto"></svg>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <!-- Main content -->
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <span class="text-sm font-medium">Table Editor</span>
                        <span class="text-xs text-muted-color">/ users</span>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [SidebarModule, Home, Database, Key, Server, Globe, Code, Cog, ExternalLink]
})
export class SidebarMultiDemo implements OnInit {
    isMobile = signal(false);

    open = signal(false);

    secondaryOpen = signal(true);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.secondaryOpen.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.secondaryOpen.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }
}
```

## Nested Menu

Menu items become collapsible to reveal a nested p-sidebar-menu-sub with deep trees and active-state tracking.

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { Sidebar } from '@primeicons/angular/sidebar';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { PIcon } from '@primeicons/angular/p-icon';

interface MenuNode {
    label: string;
    icon?: string;
    isActive?: boolean;
    children?: MenuNode[];
}

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && open()) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="nested" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [(open)]="open">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-linear-to-br from-emerald-500 to-teal-600 text-white text-xs font-bold leading-none">F</div>
                                            <span class="font-semibold text-sm">File Manager</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Files</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            @for (item of tree; track item.label) {
                                                <ng-container [ngTemplateOutlet]="node" [ngTemplateOutletContext]="{ $implicit: item }" />
                                            }
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                            <button pSidebarRail></button>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="nested" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                        <span class="text-sm font-medium">File Manager</span>
                    </header>
                    <div class="flex-1 p-6">
                        <h1 class="text-2xl font-bold mb-4">Q1 Report</h1>
                        <p class="text-muted-color">Documents &gt; Work &gt; Projects &gt; Q1 Report</p>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
        <ng-template #node let-item>
            @if (item.children?.length) {
                <p-sidebar-menu-item collapsible [defaultOpen]="hasActiveChild(item)">
                    <button pSidebarMenuButton>
                        <svg [pIcon]="item.icon"></svg>
                        <span>{{ item.label }}</span>
                        <svg data-p-icon="chevron-down" class="ml-auto transition-transform duration-200"></svg>
                    </button>
                    <p-sidebar-menu-sub>
                        @for (child of item.children; track child.label) {
                            <ng-container [ngTemplateOutlet]="node" [ngTemplateOutletContext]="{ $implicit: child }" />
                        }
                    </p-sidebar-menu-sub>
                </p-sidebar-menu-item>
            } @else {
                <p-sidebar-menu-sub-item>
                    <button pSidebarMenuSubButton [isActive]="item.isActive">
                        <span>{{ item.label }}</span>
                    </button>
                </p-sidebar-menu-sub-item>
            }
        </ng-template>
    `,
    standalone: true,
    imports: [AvatarModule, SidebarModule, ButtonModule, Sidebar, ChevronDown, PIcon]
})
export class SidebarNestedDemo implements OnInit {
    isMobile = signal(false);

    open = signal(true);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    tree: MenuNode[] = [
        {
            label: 'Documents',
            icon: 'folder',
            children: [
                {
                    label: 'Work',
                    children: [{ label: 'Projects', children: [{ label: 'Q1 Report', isActive: true }, { label: 'Q2 Report' }, { label: 'Roadmap' }] }, { label: 'Invoices' }, { label: 'Contracts' }]
                },
                { label: 'Personal', children: [{ label: 'Recipes' }, { label: 'Travel' }] }
            ]
        },
        {
            label: 'Media',
            icon: 'image',
            children: [
                { label: 'Photos', children: [{ label: '2024' }, { label: '2025' }] },
                { label: 'Videos', icon: 'video', children: [{ label: 'Tutorials' }, { label: 'Recordings' }] }
            ]
        },
        { label: 'Websites', icon: 'globe', children: [{ label: 'Blog' }, { label: 'Portfolio' }, { label: 'Documentation' }] }
    ];

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.open.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.open.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }

    hasActiveChild(item: MenuNode): boolean {
        if (item.isActive) return true;

        return item.children?.some((c) => this.hasActiveChild(c)) ?? false;
    }
}
```

## preview-doc

```typescript
import { Component, OnInit, signal } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { Plus } from '@primeicons/angular/plus';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { EllipsisV } from '@primeicons/angular/ellipsis-v';
import { Sidebar } from '@primeicons/angular/sidebar';
import { PIcon } from '@primeicons/angular/p-icon';

interface NavItem {
    icon: string;
    label: string;
    isActive?: boolean;
    badge?: string;
    subItems?: { label: string; isActive?: boolean }[];
}

interface NavGroup {
    label: string;
    action?: boolean;
    items: NavItem[];
}

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile() && open()) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="preview" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [(open)]="open">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-linear-to-br from-violet-500 to-indigo-600 text-white text-xs font-bold leading-none">A</div>
                                            <span class="font-semibold text-sm">Acme Inc</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                @for (group of navGroups; track group.label) {
                                    <p-sidebar-group>
                                        <p-sidebar-group-label>{{ group.label }}</p-sidebar-group-label>
                                        @if (group.action) {
                                            <button pSidebarGroupAction><svg data-p-icon="plus"></svg></button>
                                        }
                                        <p-sidebar-group-content>
                                            <p-sidebar-menu>
                                                @for (item of group.items; track item.label) {
                                                    <p-sidebar-menu-item [collapsible]="!!item.subItems" [defaultOpen]="hasActiveSub(item)">
                                                        <button pSidebarMenuButton [isActive]="!!item.isActive">
                                                            <svg [pIcon]="item.icon"></svg>
                                                            <span>{{ item.label }}</span>
                                                            @if (item.subItems) {
                                                                <svg data-p-icon="chevron-down" class="ml-auto"></svg>
                                                            }
                                                        </button>
                                                        @if (item.badge) {
                                                            <p-sidebar-menu-badge>{{ item.badge }}</p-sidebar-menu-badge>
                                                        }
                                                        @if (item.subItems) {
                                                            <p-sidebar-menu-sub>
                                                                @for (sub of item.subItems; track sub.label) {
                                                                    <p-sidebar-menu-sub-item>
                                                                        <button pSidebarMenuSubButton [isActive]="!!sub.isActive">
                                                                            <span>{{ sub.label }}</span>
                                                                        </button>
                                                                    </p-sidebar-menu-sub-item>
                                                                }
                                                            </p-sidebar-menu-sub>
                                                        } @else if (!item.badge) {
                                                            <button pSidebarMenuAction showOnHover><svg data-p-icon="ellipsis-v"></svg></button>
                                                        }
                                                    </p-sidebar-menu-item>
                                                }
                                            </p-sidebar-menu>
                                        </p-sidebar-group-content>
                                    </p-sidebar-group>
                                }
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                            <button pSidebarRail></button>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="preview" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, SidebarModule, ButtonModule, Plus, ChevronDown, EllipsisV, Sidebar, PIcon]
})
export class SidebarPreviewDemo implements OnInit {
    isMobile = signal(false);

    open = signal(true);

    private mql?: MediaQueryList;

    private mqlListener?: (e: MediaQueryListEvent) => void;

    navGroups: NavGroup[] = [
        {
            label: 'Navigation',
            items: [
                { icon: 'home', label: 'Home', isActive: true },
                { icon: 'inbox', label: 'Inbox', badge: '12' },
                { icon: 'search', label: 'Search' },
                { icon: 'bell', label: 'Notifications', badge: '3' }
            ]
        },
        {
            label: 'Projects',
            action: true,
            items: [
                { icon: 'chart-bar', label: 'Analytics', subItems: [{ label: 'Overview', isActive: true }, { label: 'Reports' }, { label: 'Real-time' }] },
                { icon: 'users', label: 'Team' },
                { icon: 'calendar', label: 'Calendar' },
                { icon: 'folder', label: 'Documents', subItems: [{ label: 'Shared' }, { label: 'Private' }, { label: 'Archived' }] }
            ]
        },
        {
            label: 'Billing',
            items: [
                { icon: 'credit-card', label: 'Payments' },
                { icon: 'shopping-cart', label: 'Orders' },
                { icon: 'star', label: 'Subscriptions' }
            ]
        }
    ];

    ngOnInit() {
        if (typeof window === 'undefined') return;
        this.mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(this.mql.matches);
        this.open.set(!this.mql.matches);
        this.mqlListener = (e) => {
            this.isMobile.set(e.matches);
            this.open.set(!e.matches);
        };
        this.mql.addEventListener('change', this.mqlListener);
    }

    ngOnDestroy() {
        this.mql?.removeEventListener('change', this.mqlListener!);
    }

    hasActiveSub(item: NavItem): boolean {
        return !!item.subItems?.some((s) => s.isActive);
    }
}
```

## Responsive

Below 1024px the sidebar turns into an offcanvas overlay with a backdrop. Above that it stays in icon mode pushing the inset.

```typescript
import { Component, signal } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { Home } from '@primeicons/angular/home';
import { Inbox } from '@primeicons/angular/inbox';
import { Search } from '@primeicons/angular/search';
import { Users } from '@primeicons/angular/users';
import { Bell } from '@primeicons/angular/bell';
import { Cog } from '@primeicons/angular/cog';
import { Sidebar } from '@primeicons/angular/sidebar';

@Component({
    template: `
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (isMobile()) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                <p-sidebar id="mobile-nav" [collapsible]="isMobile() ? 'offcanvas' : 'icon'" [overlay]="isMobile()" [open]="!isMobile()" width="14rem">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-linear-to-br from-violet-500 to-indigo-600 text-white text-xs font-bold leading-none">A</div>
                                            <span class="font-semibold text-sm">Acme Inc</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                <p-sidebar-group>
                                    <p-sidebar-group-label>Menu</p-sidebar-group-label>
                                    <p-sidebar-group-content>
                                        <p-sidebar-menu>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton [isActive]="true">
                                                    <svg data-p-icon="home"></svg>
                                                    <span>Home</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="inbox"></svg>
                                                    <span>Inbox</span>
                                                </button>
                                                <p-sidebar-menu-badge>3</p-sidebar-menu-badge>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="search"></svg>
                                                    <span>Search</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="users"></svg>
                                                    <span>Team</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="bell"></svg>
                                                    <span>Notifications</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                            <p-sidebar-menu-item>
                                                <button pSidebarMenuButton>
                                                    <svg data-p-icon="cog"></svg>
                                                    <span>Settings</span>
                                                </button>
                                            </p-sidebar-menu-item>
                                        </p-sidebar-menu>
                                    </p-sidebar-group-content>
                                </p-sidebar-group>
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                <p-sidebar-main>
                    <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                        <button pButton pSidebarTrigger target="mobile-nav" severity="secondary" text size="small">
                            <svg data-p-icon="sidebar"></svg>
                        </button>
                        <span class="text-sm font-medium">Dashboard</span>
                        <span class="ml-auto text-xs text-muted-color rounded-md bg-surface-100 dark:bg-surface-800 px-2 py-1">{{ isMobile() ? 'Mobile' : 'Desktop' }}</span>
                    </header>
                    <div class="flex-1 p-4 flex flex-col gap-4">
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                        <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                    </div>
                </p-sidebar-main>
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, SidebarModule, ButtonModule, Home, Inbox, Search, Users, Bell, Cog, Sidebar]
})
export class SidebarResponsiveDemo {
    isMobile = signal(false);

    constructor() {
        if (typeof window === 'undefined') return;
        const mql = window.matchMedia('(max-width: 1023px)');
        this.isMobile.set(mql.matches);
        mql.addEventListener('change', (e) => this.isMobile.set(e.matches));
    }
}
```

## Variants

Interactive playground for every sidebar variant, collapsible mode, side, overlay, open-on-hover and backdrop.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { SelectModule } from 'primeng/select';
import { SelectButtonModule } from 'primeng/selectbutton';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { SidebarModule } from 'primeng/sidebar';
import { ButtonModule } from 'primeng/button';
import { LabelModule } from 'primeng/label';
import { SidebarCollapsible, SidebarSide, SidebarVariant } from 'primeng/types/sidebar';
import { Sidebar } from '@primeicons/angular/sidebar';
import { ChevronDown } from '@primeicons/angular/chevron-down';
import { EllipsisV } from '@primeicons/angular/ellipsis-v';
import { PIcon } from '@primeicons/angular/p-icon';

interface NavItem {
    icon: string;
    label: string;
    isActive?: boolean;
    badge?: string;
    subItems?: { label: string; isActive?: boolean }[];
}

interface NavGroup {
    label: string;
    items: NavItem[];
}

@Component({
    template: `
        <div class="flex flex-wrap items-start gap-4 mb-4">
            <div class="flex flex-col gap-1.5">
                <label pLabel>Variant</label>
                <p-select [options]="variantOptions" [(ngModel)]="variant" optionLabel="label" optionValue="value" placeholder="Variant" styleClass="w-40" />
            </div>
            <div class="flex flex-col gap-1.5">
                <label pLabel>Collapsible</label>
                <p-select [options]="collapsibleOptions" [(ngModel)]="collapsible" optionLabel="label" optionValue="value" placeholder="Collapsible" styleClass="w-40" />
            </div>
            <div class="flex flex-col gap-1.5">
                <label pLabel>Side</label>
                <p-selectbutton [options]="sideOptions" [(ngModel)]="side" optionLabel="label" optionValue="value" [allowEmpty]="false" />
            </div>
            <div class="flex flex-col gap-1.5">
                <label pLabel for="overlay-sw">Overlay</label>
                <p-toggleswitch inputId="overlay-sw" [(ngModel)]="overlay" />
            </div>
            <div class="flex flex-col gap-1.5">
                <label pLabel for="hover-sw">Open on Hover</label>
                <p-toggleswitch inputId="hover-sw" [(ngModel)]="openOnHover" />
            </div>
            <div class="flex flex-col gap-1.5">
                <label pLabel for="backdrop-sw">Backdrop</label>
                <p-toggleswitch inputId="backdrop-sw" [(ngModel)]="backdrop" />
            </div>
        </div>
        <div class="border border-surface-200 dark:border-surface-700 rounded-lg overflow-hidden">
            <p-sidebar-layout class="min-h-192! relative!">
                @if (backdrop) {
                    <p-sidebar-backdrop class="absolute!" />
                }
                @if (side === 'right') {
                    <p-sidebar-main>
                        <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                            <button pButton pSidebarTrigger target="variants-demo" severity="secondary" text size="small">
                                <svg data-p-icon="sidebar"></svg>
                            </button>
                        </header>
                        <div class="flex-1 p-4 flex flex-col gap-4">
                            <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                            <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                        </div>
                    </p-sidebar-main>
                }
                <p-sidebar id="variants-demo" [variant]="variant" [collapsible]="collapsible" [side]="side" [overlay]="overlay" [openOnHover]="openOnHover">
                    <p-sidebar-spacer />
                    <p-sidebar-aside>
                        <p-sidebar-panel>
                            <p-sidebar-header>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="px-1!">
                                            <div class="flex size-6 shrink-0 items-center justify-center rounded-md bg-linear-to-br from-violet-500 to-indigo-600 text-white text-xs font-bold leading-none">A</div>
                                            <span class="font-semibold text-sm">Acme Inc</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-header>
                            <p-sidebar-content>
                                @for (group of navGroups; track group.label) {
                                    <p-sidebar-group>
                                        <p-sidebar-group-label>{{ group.label }}</p-sidebar-group-label>
                                        <p-sidebar-group-content>
                                            <p-sidebar-menu>
                                                @for (item of group.items; track item.label) {
                                                    <p-sidebar-menu-item [collapsible]="!!item.subItems" [defaultOpen]="hasActiveSub(item)">
                                                        <button pSidebarMenuButton [isActive]="!!item.isActive">
                                                            <svg [pIcon]="item.icon"></svg>
                                                            <span>{{ item.label }}</span>
                                                            @if (item.subItems) {
                                                                <svg data-p-icon="chevron-down" class="ml-auto"></svg>
                                                            }
                                                        </button>
                                                        @if (item.badge) {
                                                            <p-sidebar-menu-badge>{{ item.badge }}</p-sidebar-menu-badge>
                                                        }
                                                        @if (item.subItems) {
                                                            <p-sidebar-menu-sub>
                                                                @for (sub of item.subItems; track sub.label) {
                                                                    <p-sidebar-menu-sub-item>
                                                                        <button pSidebarMenuSubButton [isActive]="!!sub.isActive">
                                                                            <span>{{ sub.label }}</span>
                                                                        </button>
                                                                    </p-sidebar-menu-sub-item>
                                                                }
                                                            </p-sidebar-menu-sub>
                                                        } @else if (!item.badge) {
                                                            <button pSidebarMenuAction showOnHover><svg data-p-icon="ellipsis-v"></svg></button>
                                                        }
                                                    </p-sidebar-menu-item>
                                                }
                                            </p-sidebar-menu>
                                        </p-sidebar-group-content>
                                    </p-sidebar-group>
                                }
                            </p-sidebar-content>
                            <p-sidebar-footer>
                                <p-sidebar-menu>
                                    <p-sidebar-menu-item>
                                        <button pSidebarMenuButton class="p-1!">
                                            <p-avatar label="JD" shape="circle" class="size-6 shrink-0 text-xs" />
                                            <span>John Doe</span>
                                        </button>
                                    </p-sidebar-menu-item>
                                </p-sidebar-menu>
                            </p-sidebar-footer>
                            <button pSidebarRail></button>
                        </p-sidebar-panel>
                    </p-sidebar-aside>
                </p-sidebar>
                @if (side === 'left') {
                    <p-sidebar-main>
                        <header class="flex h-12 items-center gap-2 border-b border-surface-200 dark:border-surface-700 px-4">
                            <button pButton pSidebarTrigger target="variants-demo" severity="secondary" text size="small">
                                <svg data-p-icon="sidebar"></svg>
                            </button>
                        </header>
                        <div class="flex-1 p-4 flex flex-col gap-4">
                            <div class="rounded-lg bg-surface-100 dark:bg-surface-800 h-48"></div>
                            <div class="rounded-lg bg-surface-100 dark:bg-surface-800 flex-1"></div>
                        </div>
                    </p-sidebar-main>
                }
            </p-sidebar-layout>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, SelectModule, SelectButtonModule, ToggleSwitchModule, SidebarModule, ButtonModule, LabelModule, FormsModule, Sidebar, ChevronDown, EllipsisV, PIcon]
})
export class SidebarVariantsDemo {
    variant: SidebarVariant = 'sidebar';

    collapsible: SidebarCollapsible = 'icon';

    side: SidebarSide = 'left';

    overlay: boolean = false;

    openOnHover: boolean = false;

    backdrop: boolean = false;

    variantOptions = [
        { label: 'Sidebar', value: 'sidebar' },
        { label: 'Floating', value: 'floating' },
        { label: 'Inset', value: 'inset' }
    ];

    collapsibleOptions = [
        { label: 'Icon', value: 'icon' },
        { label: 'Offcanvas', value: 'offcanvas' },
        { label: 'None', value: 'none' }
    ];

    sideOptions = [
        { label: 'Left', value: 'left' },
        { label: 'Right', value: 'right' }
    ];

    navGroups: NavGroup[] = [
        {
            label: 'Navigation',
            items: [
                { icon: 'home', label: 'Home', isActive: true },
                { icon: 'inbox', label: 'Inbox', badge: '12' },
                { icon: 'search', label: 'Search' },
                { icon: 'bell', label: 'Notifications', badge: '3' }
            ]
        },
        {
            label: 'Projects',
            items: [
                { icon: 'chart-bar', label: 'Analytics', subItems: [{ label: 'Overview', isActive: true }, { label: 'Reports' }, { label: 'Real-time' }] },
                { icon: 'users', label: 'Team' },
                { icon: 'calendar', label: 'Calendar' },
                { icon: 'folder', label: 'Documents', subItems: [{ label: 'Shared' }, { label: 'Private' }, { label: 'Archived' }] }
            ]
        },
        {
            label: 'Billing',
            items: [
                { icon: 'credit-card', label: 'Payments' },
                { icon: 'shopping-cart', label: 'Orders' },
                { icon: 'star', label: 'Subscriptions' }
            ]
        }
    ];

    hasActiveSub(item: NavItem): boolean {
        return !!item.subItems?.some((s) => s.isActive);
    }
}
```

## Sidebar

Sidebar is a compound navigation panel with collapsible icon mode, offcanvas mode, variants and optional overlay.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SidebarPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| open | boolean | true | Two-way bound open state. |
| side | "left" \| "right" | 'left' | Side the sidebar attaches to. |
| variant | "sidebar" \| "floating" \| "inset" | 'sidebar' | Visual variant. |
| collapsible | "offcanvas" \| "icon" \| "none" | 'icon' | Collapse mode. |
| overlay | boolean | false | When enabled, the sidebar overlays the inset instead of pushing it. |
| dismissable | boolean | true | Clicking the backdrop closes the sidebar when overlay is active. |
| hideOnOutsideClick | boolean | true | When overlay is active, clicking inside the SidebarMain area collapses the sidebar. Clicks originating from a [pSidebarTrigger] are excluded so opening via the trigger is not immediately undone. |
| width | string | '16rem' | Expanded width (CSS length). |
| iconWidth | string | '3rem' | Icon-mode width (CSS length). |
| openOnHover | boolean | false | Open/close the sidebar on pointer enter/leave of the root. |
| hoverOpenDelay | number | 50 | Delay in ms before opening on hover. |
| hoverCloseDelay | number | 100 | Delay in ms before closing on hover leave. |
| id | string | - | Unique id used by [pSidebarTrigger] target lookup. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> |  |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-sidebar | Class name of the root element |
| p-sidebar-layout | Class name of the layout element |
| p-sidebar-spacer | Class name of the spacer element |
| p-sidebar-aside | Class name of the aside element |
| p-sidebar-panel | Class name of the panel element |
| p-sidebar-header | Class name of the header element |
| p-sidebar-content | Class name of the content element |
| p-sidebar-footer | Class name of the footer element |
| p-sidebar-group | Class name of the group element |
| p-sidebar-group-label | Class name of the group label element |
| p-sidebar-group-action | Class name of the group action element |
| p-sidebar-group-content | Class name of the group content element |
| p-sidebar-menu | Class name of the menu element |
| p-sidebar-menu-item | Class name of the menu item element |
| p-sidebar-menu-button | Class name of the menu button element |
| p-sidebar-menu-action | Class name of the menu action element |
| p-sidebar-menu-badge | Class name of the menu badge element |
| p-sidebar-menu-sub | Class name of the menu sub element |
| p-sidebar-menu-sub-item | Class name of the menu sub item element |
| p-sidebar-menu-sub-button | Class name of the menu sub button element |
| p-sidebar-trigger | Class name of the trigger element |
| p-sidebar-rail | Class name of the rail element |
| p-sidebar-main | Class name of the main element |
| p-sidebar-backdrop | Class name of the backdrop element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| sidebar.border.color | --p-sidebar-border-color | Border color of root |
| sidebar.focus.ring.width | --p-sidebar-focus-ring-width | Focus ring width of root |
| sidebar.focus.ring.style | --p-sidebar-focus-ring-style | Focus ring style of root |
| sidebar.focus.ring.color | --p-sidebar-focus-ring-color | Focus ring color of root |
| sidebar.focus.ring.offset | --p-sidebar-focus-ring-offset | Focus ring offset of root |
| sidebar.focus.ring.shadow | --p-sidebar-focus-ring-shadow | Focus ring shadow of root |
| sidebar.layout.background | --p-sidebar-layout-background | Background of layout |
| sidebar.header.padding | --p-sidebar-header-padding | Padding of header |
| sidebar.header.gap | --p-sidebar-header-gap | Gap of header |
| sidebar.footer.padding | --p-sidebar-footer-padding | Padding of footer |
| sidebar.footer.gap | --p-sidebar-footer-gap | Gap of footer |
| sidebar.panel.background | --p-sidebar-panel-background | Background of panel |
| sidebar.panel.color | --p-sidebar-panel-color | Color of panel |
| sidebar.panel.floating.border.radius | --p-sidebar-panel-floating-border-radius | Border radius of floating variant panel |
| sidebar.panel.floating.shadow | --p-sidebar-panel-floating-shadow | Shadow of floating variant panel |
| sidebar.content.gap | --p-sidebar-content-gap | Gap of content |
| sidebar.aside.padding | --p-sidebar-aside-padding | Padding of aside |
| sidebar.group.padding | --p-sidebar-group-padding | Padding of group |
| sidebar.group.label.padding | --p-sidebar-group-label-padding | Padding of group label |
| sidebar.group.label.height | --p-sidebar-group-label-height | Height of group label |
| sidebar.group.label.border.radius | --p-sidebar-group-label-border-radius | Border radius of group label |
| sidebar.group.label.font.size | --p-sidebar-group-label-font-size | Font size of group label |
| sidebar.group.label.font.weight | --p-sidebar-group-label-font-weight | Font weight of group label |
| sidebar.group.label.color | --p-sidebar-group-label-color | Color of group label |
| sidebar.group.action.top | --p-sidebar-group-action-top | Top of group action |
| sidebar.group.action.right | --p-sidebar-group-action-right | Right of group action |
| sidebar.group.action.size | --p-sidebar-group-action-size | Size of group action |
| sidebar.group.action.border.radius | --p-sidebar-group-action-border-radius | Border radius of group action |
| sidebar.group.action.color | --p-sidebar-group-action-color | Color of group action |
| sidebar.group.action.focus.color | --p-sidebar-group-action-focus-color | Focus color of group action |
| sidebar.group.action.focus.background | --p-sidebar-group-action-focus-background | Focus background of group action |
| sidebar.group.action.icon.size | --p-sidebar-group-action-icon-size | Icon size of group action |
| sidebar.menu.gap | --p-sidebar-menu-gap | Gap of menu |
| sidebar.menu.button.padding | --p-sidebar-menu-button-padding | Padding of menu button |
| sidebar.menu.button.gap | --p-sidebar-menu-button-gap | Gap of menu button |
| sidebar.menu.button.height | --p-sidebar-menu-button-height | Height of menu button |
| sidebar.menu.button.border.radius | --p-sidebar-menu-button-border-radius | Border radius of menu button |
| sidebar.menu.button.font.size | --p-sidebar-menu-button-font-size | Font size of menu button |
| sidebar.menu.button.font.weight | --p-sidebar-menu-button-font-weight | Font weight of menu button |
| sidebar.menu.button.color | --p-sidebar-menu-button-color | Color of menu button |
| sidebar.menu.button.focus.background | --p-sidebar-menu-button-focus-background | Focus background of menu button |
| sidebar.menu.button.focus.color | --p-sidebar-menu-button-focus-color | Focus color of menu button |
| sidebar.menu.button.active.background | --p-sidebar-menu-button-active-background | Active background of menu button |
| sidebar.menu.button.active.color | --p-sidebar-menu-button-active-color | Active color of menu button |
| sidebar.menu.button.icon.only.width | --p-sidebar-menu-button-icon-only-width | Width of menu button in icon-only collapsible mode |
| sidebar.menu.button.with.action.padding.end | --p-sidebar-menu-button-with-action-padding-end | Padding end of menu button when it has a sibling menu action |
| sidebar.menu.button.icon.color | --p-sidebar-menu-button-icon-color | Icon color of menu button |
| sidebar.menu.button.icon.focus.color | --p-sidebar-menu-button-icon-focus-color | Icon focus color of menu button |
| sidebar.menu.button.icon.size | --p-sidebar-menu-button-icon-size | Icon size of menu button |
| sidebar.menu.action.top | --p-sidebar-menu-action-top | Top of menu action |
| sidebar.menu.action.right | --p-sidebar-menu-action-right | Right of menu action |
| sidebar.menu.action.width | --p-sidebar-menu-action-width | Width of menu action |
| sidebar.menu.action.border.radius | --p-sidebar-menu-action-border-radius | Border radius of menu action |
| sidebar.menu.action.color | --p-sidebar-menu-action-color | Color of menu action |
| sidebar.menu.action.focus.color | --p-sidebar-menu-action-focus-color | Focus color of menu action |
| sidebar.menu.action.focus.background | --p-sidebar-menu-action-focus-background | Focus background of menu action |
| sidebar.menu.action.icon.size | --p-sidebar-menu-action-icon-size | Icon size of menu action |
| sidebar.menu.badge.top | --p-sidebar-menu-badge-top | Top of menu badge |
| sidebar.menu.badge.right | --p-sidebar-menu-badge-right | Right of menu badge |
| sidebar.menu.badge.height | --p-sidebar-menu-badge-height | Height of menu badge |
| sidebar.menu.badge.min.width | --p-sidebar-menu-badge-min-width | Min width of menu badge |
| sidebar.menu.badge.border.radius | --p-sidebar-menu-badge-border-radius | Border radius of menu badge |
| sidebar.menu.badge.padding | --p-sidebar-menu-badge-padding | Padding of menu badge |
| sidebar.menu.badge.font.size | --p-sidebar-menu-badge-font-size | Font size of menu badge |
| sidebar.menu.badge.font.weight | --p-sidebar-menu-badge-font-weight | Font weight of menu badge |
| sidebar.menu.badge.background | --p-sidebar-menu-badge-background | Background of menu badge |
| sidebar.menu.badge.border.color | --p-sidebar-menu-badge-border-color | Border color of menu badge |
| sidebar.menu.badge.color | --p-sidebar-menu-badge-color | Color of menu badge |
| sidebar.menu.sub.padding.block | --p-sidebar-menu-sub-padding-block | Block padding of menu sub |
| sidebar.menu.sub.gap | --p-sidebar-menu-sub-gap | Gap of menu sub |
| sidebar.menu.sub.indent.margin | --p-sidebar-menu-sub-indent-margin | Margin of indented (non-collapsible) menu sub |
| sidebar.menu.sub.indent.padding | --p-sidebar-menu-sub-indent-padding | Padding of indented (non-collapsible) menu sub |
| sidebar.menu.sub.collapsible.indent | --p-sidebar-menu-sub-collapsible-indent | Left indent of collapsible menu sub |
| sidebar.menu.sub.collapsible.top.margin | --p-sidebar-menu-sub-collapsible-top-margin | Top margin of collapsible menu sub |
| sidebar.menu.sub.collapsible.border.radius | --p-sidebar-menu-sub-collapsible-border-radius | Border radius of collapsible menu sub |
| sidebar.menu.sub.button.padding | --p-sidebar-menu-sub-button-padding | Padding of menu sub button |
| sidebar.menu.sub.button.gap | --p-sidebar-menu-sub-button-gap | Gap of menu sub button |
| sidebar.menu.sub.button.height | --p-sidebar-menu-sub-button-height | Height of menu sub button |
| sidebar.menu.sub.button.border.radius | --p-sidebar-menu-sub-button-border-radius | Border radius of menu sub button |
| sidebar.menu.sub.button.font.size | --p-sidebar-menu-sub-button-font-size | Font size of menu sub button |
| sidebar.menu.sub.button.font.weight | --p-sidebar-menu-sub-button-font-weight | Font weight of menu sub button |
| sidebar.menu.sub.button.color | --p-sidebar-menu-sub-button-color | Color of menu sub button |
| sidebar.menu.sub.button.focus.background | --p-sidebar-menu-sub-button-focus-background | Focus background of menu sub button |
| sidebar.menu.sub.button.focus.color | --p-sidebar-menu-sub-button-focus-color | Focus color of menu sub button |
| sidebar.menu.sub.button.active.background | --p-sidebar-menu-sub-button-active-background | Active background of menu sub button |
| sidebar.menu.sub.button.active.color | --p-sidebar-menu-sub-button-active-color | Active color of menu sub button |
| sidebar.menu.sub.button.icon.color | --p-sidebar-menu-sub-button-icon-color | Icon color of menu sub button |
| sidebar.menu.sub.button.icon.focus.color | --p-sidebar-menu-sub-button-icon-focus-color | Icon focus color of menu sub button |
| sidebar.menu.sub.button.icon.size | --p-sidebar-menu-sub-button-icon-size | Icon size of menu sub button |
| sidebar.main.background | --p-sidebar-main-background | Background of main |
| sidebar.main.floating.background | --p-sidebar-main-floating-background | Floating background of main |
| sidebar.main.inset.background | --p-sidebar-main-inset-background | Inset background of main |
| sidebar.main.margin | --p-sidebar-main-margin | Margin of main |
| sidebar.main.border.radius | --p-sidebar-main-border-radius | Border radius of main |
| sidebar.main.shadow | --p-sidebar-main-shadow | Shadow of main |
