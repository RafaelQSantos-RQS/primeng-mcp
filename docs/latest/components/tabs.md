---
title: "Angular Tabs Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/tabs.md"
description: "Tabs is a container component to group content with tabs."
---

# Angular Tabs Component

Tabs is a container component to group content with tabs.

## Accessibility

Screen Reader Tabs container is defined with the tablist role, as any attribute is passed to the container element aria-labelledby can be optionally used to specify an element to describe the Tabs. Each tab header has a tab role along with aria-selected state attribute and aria-controls to refer to the corresponding tab content element. The content element of each tab has tabpanel role, an id to match the aria-controls of the header and aria-labelledby reference to the header as the accessible name. Tab Header Keyboard Support Key Function tab Moves focus through the header. enter Activates the focused tab header. space Activates the focused tab header. right arrow Moves focus to the next header. left arrow Moves focus to the previous header. home Moves focus to the last header. end Moves focus to the first header.

## Basic

Organizes content into selectable, horizontally laid out sections.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="tab1">
            <p-tablist>
                <p-tab value="tab1">Account Info</p-tab>
                <p-tab value="tab2">Payment</p-tab>
                <p-tab value="tab3">Preferences</p-tab>
            </p-tablist>
            <p-tabpanels>
                <p-tabpanel value="tab1">
                    <h2 class="text-lg font-bold">Account Info</h2>
                    <p class="text-surface-500 mt-1">Update your personal information such as name, email address, and profile picture.</p>
                </p-tabpanel>
                <p-tabpanel value="tab2">
                    <h2 class="text-lg font-bold">Payment</h2>
                    <p class="text-surface-500 mt-1">Manage your subscription plan, view invoices, and update your payment method.</p>
                </p-tabpanel>
                <p-tabpanel value="tab3">
                    <h2 class="text-lg font-bold">Preferences</h2>
                    <p class="text-surface-500 mt-1">Customize how the application looks and behaves to match your personal preferences.</p>
                </p-tabpanel>
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsBasicDemo {}
```

## Controlled

Control the active tab with the value property.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="space-y-4">
            <button type="button" pButton (click)="value = 'tab2'">Go to Payment</button>
            <p-tabs [(value)]="value">
                <p-tablist>
                    @for (tab of tabs; track tab.id) {
                        <p-tab [value]="tab.id">{{ tab.title }}</p-tab>
                    }
                </p-tablist>
                <p-tabpanels>
                    @for (tab of tabs; track tab.id) {
                        <p-tabpanel [value]="tab.id">
                            <h2 class="text-lg font-bold">{{ tab.title }}</h2>
                            <p class="text-surface-500 mt-1">{{ tab.content }}</p>
                        </p-tabpanel>
                    }
                </p-tabpanels>
            </p-tabs>
        </div>
    `,
    standalone: true,
    imports: [TabsModule, ButtonModule]
})
export class TabsControlledDemo {
    value: string = 'tab1';

    tabs = [
        { id: 'tab1', title: 'Account Info', content: 'Update your personal information such as name, email address, and profile picture.' },
        { id: 'tab2', title: 'Payment', content: 'Manage your subscription plan, view invoices, and update your payment method.' },
        { id: 'tab3', title: 'Preferences', content: 'Customize how the application looks and behaves to match your personal preferences.' }
    ];
}
```

## customindicator-doc

Customize the active tab highlight styles using the activeBar pass through option.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="tab1">
            <p-tablist
                [pt]="{
                    activeBar: {
                        class: 'w-[calc(var(--px-active-bar-width)-10px)]! h-[calc(var(--px-active-bar-height)-16px)]! left-[calc(var(--px-active-bar-left)+5px)]! top-[calc(var(--px-active-bar-top)+8px)]! bg-surface-100! dark:bg-surface-800! rounded-md! transition-[left,width]! duration-200! border-0!'
                    }
                }"
            >
                @for (tab of tabs; track tab.id) {
                    <p-tab [value]="tab.id" class="border-none z-10">{{ tab.title }}</p-tab>
                }
            </p-tablist>
            <p-tabpanels>
                @for (tab of tabs; track tab.id) {
                    <p-tabpanel [value]="tab.id">
                        <h2 class="text-lg font-bold">{{ tab.title }}</h2>
                        <p class="text-surface-500 mt-1">{{ tab.content }}</p>
                    </p-tabpanel>
                }
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsCustomindicatorDemo {
    tabs = [
        { id: 'tab1', title: 'Account Info', content: 'Update your personal information such as name, email address, and profile picture.' },
        { id: 'tab2', title: 'Payment', content: 'Manage your subscription plan, view invoices, and update your payment method.' },
        { id: 'tab3', title: 'Preferences', content: 'Customize how the application looks and behaves to match your personal preferences.' }
    ];
}
```

## customtemplate-doc

Use custom markup inside tabs and panels to build richer tab content.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { BadgeModule } from 'primeng/badge';
import { TabsModule } from 'primeng/tabs';
import { ToggleSwitchModule } from 'primeng/toggleswitch';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';
import { User } from '@primeicons/angular/user';
import { CreditCard } from '@primeicons/angular/credit-card';
import { Cog } from '@primeicons/angular/cog';

@Component({
    template: `
        <p-tabs value="tab1" class="max-w-md mx-auto">
            <p-tablist>
                <p-tab value="tab1" class="flex items-center gap-2!">
                    <svg data-p-icon="user"></svg>
                    Account Info
                </p-tab>
                <p-tab value="tab2" class="flex items-center gap-2!">
                    <svg data-p-icon="credit-card"></svg>
                    Payment
                    <p-badge size="small" value="New" />
                </p-tab>
                <p-tab value="tab3" class="flex items-center gap-2!">
                    <svg data-p-icon="cog"></svg>
                    Preferences
                </p-tab>
            </p-tablist>
            <p-tabpanels>
                <p-tabpanel value="tab1">
                    <div>
                        <p class="mt-2 mb-8 text-surface-500">Update your personal information such as name, email address, and profile picture.</p>
                        <div class="space-y-4">
                            <div class="flex flex-col gap-1">
                                <label pLabel for="username">Username</label>
                                <input pInputText id="username" placeholder="john.doe" />
                            </div>
                            <div class="flex flex-col gap-1">
                                <label pLabel for="email">Email</label>
                                <input pInputText id="email" placeholder="john.doe&#64;example.com" />
                            </div>
                        </div>
                        <button type="button" pButton class="mt-8 w-fit">Save Changes</button>
                    </div>
                </p-tabpanel>
                <p-tabpanel value="tab2">
                    <div>
                        <p class="mt-2 mb-8 text-surface-500">Manage your subscription plan, view invoices, and update your payment method.</p>
                        <div class="space-y-4">
                            <div class="flex flex-col gap-1">
                                <label pLabel for="cardName">Cardholder Name</label>
                                <input pInputText id="cardName" placeholder="John Doe" />
                            </div>
                            <div class="flex flex-col gap-1">
                                <label pLabel for="cardNumber">Card Number</label>
                                <input pInputText id="cardNumber" placeholder="0000 0000 0000 0000" />
                            </div>
                            <div class="flex flex-col gap-1">
                                <label pLabel for="expiryDate">Expiry Date</label>
                                <input pInputText id="expiryDate" placeholder="MM/YY" />
                            </div>
                        </div>
                        <button type="button" pButton class="mt-8 w-fit">Update Payment</button>
                    </div>
                </p-tabpanel>
                <p-tabpanel value="tab3">
                    <div>
                        <p class="mt-2 mb-8 text-surface-500">Customize how the application looks and behaves to match your personal preferences.</p>
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <label pLabel for="darkMode">Dark Mode</label>
                                <p-toggleswitch inputId="darkMode" [(ngModel)]="darkMode" />
                            </div>
                            <div class="flex items-center justify-between">
                                <label pLabel for="emailNotifications">Email Notifications</label>
                                <p-toggleswitch inputId="emailNotifications" [(ngModel)]="emailNotifications" />
                            </div>
                            <div class="flex items-center justify-between">
                                <label pLabel for="desktopNotifications">Desktop Notifications</label>
                                <p-toggleswitch inputId="desktopNotifications" [(ngModel)]="desktopNotifications" />
                            </div>
                        </div>
                        <button type="button" pButton class="w-fit mt-8 ml-auto mr-0">Save Preferences</button>
                    </div>
                </p-tabpanel>
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [BadgeModule, TabsModule, ToggleSwitchModule, ButtonModule, InputTextModule, LabelModule, FormsModule, User, CreditCard, Cog]
})
export class TabsCustomTemplateDemo {
    darkMode: boolean = false;

    emailNotifications: boolean = true;

    desktopNotifications: boolean = false;
}
```

## Disabled

Set disabled on a Tab to prevent selection.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="tab1">
            <p-tablist>
                <p-tab value="tab1">Account Info</p-tab>
                <p-tab value="tab2" disabled>Payment</p-tab>
                <p-tab value="tab3">Preferences</p-tab>
            </p-tablist>
            <p-tabpanels>
                <p-tabpanel value="tab1">
                    <h2 class="text-lg font-bold">Account Info</h2>
                    <p class="text-surface-500 mt-1">Update your personal information such as name, email address, and profile picture.</p>
                </p-tabpanel>
                <p-tabpanel value="tab2">
                    <h2 class="text-lg font-bold">Payment</h2>
                    <p class="text-surface-500 mt-1">Manage your subscription plan, view invoices, and update your payment method.</p>
                </p-tabpanel>
                <p-tabpanel value="tab3">
                    <h2 class="text-lg font-bold">Preferences</h2>
                    <p class="text-surface-500 mt-1">Customize how the application looks and behaves to match your personal preferences.</p>
                </p-tabpanel>
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsDisabledDemo {}
```

## Dynamic

Create tabs from an array to keep labels and panel content in sync.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="tab1">
            <p-tablist>
                @for (tab of tabs; track tab.id) {
                    <p-tab [value]="tab.id">{{ tab.title }}</p-tab>
                }
            </p-tablist>
            <p-tabpanels>
                @for (tab of tabs; track tab.id) {
                    <p-tabpanel [value]="tab.id">
                        <h2 class="text-lg font-bold">{{ tab.title }}</h2>
                        <p class="text-surface-500 mt-1">{{ tab.content }}</p>
                    </p-tabpanel>
                }
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsDynamicDemo {
    tabs = [
        { id: 'tab1', title: 'Account Info', content: 'Update your personal information such as name, email address, and profile picture.' },
        { id: 'tab2', title: 'Payment', content: 'Manage your subscription plan, view invoices, and update your payment method.' },
        { id: 'tab3', title: 'Preferences', content: 'Customize how the application looks and behaves to match your personal preferences.' }
    ];
}
```

## Lazy

By default, inactive tab's content is rendered (but hidden). You can use the lazy input (either globally on Tabs or individually on a TabPanel ) to change this behavior so that content is only rendered when the tab becomes active. If a lazy tab contains complex components that should only be initialized when the tab is activated, you should wrap the content inside: &lt;ng-template #content&gt;your content&lt;/ng-template&gt; .

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs lazy value="0">
            <p-tablist>
                <p-tab value="0">Header I</p-tab>
                <p-tab value="1">Header II</p-tab>
                <p-tab value="2">Header III</p-tab>
            </p-tablist>
            <p-tabpanels>
                <p-tabpanel value="0">
                    <p class="m-0 text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                        consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est
                        laborum.
                    </p>
                </p-tabpanel>
                <p-tabpanel value="1">
                    <p class="m-0 text-sm">
                        Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo
                        enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Consectetur, adipisci velit, sed quia non numquam eius modi.
                    </p>
                </p-tabpanel>
                <p-tabpanel value="2">
                    <ng-template #content>Complex components that should only be initialized when the tab becomes active</ng-template>
                </p-tabpanel>
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsLazyDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';
import { Code } from '@primeicons/angular/code';
import { InfoCircle } from '@primeicons/angular/info-circle';
import { CodeBranch } from '@primeicons/angular/code-branch';
import { Cog } from '@primeicons/angular/cog';

@Component({
    template: `
        <p-tabs value="tab1">
            <p-tablist>
                <p-tab value="tab1" class="flex items-center gap-2!">
                    <svg data-p-icon="code"></svg>
                    Code
                </p-tab>
                <p-tab value="tab2" class="flex items-center gap-2!">
                    <svg data-p-icon="info-circle"></svg>
                    Issues
                </p-tab>
                <p-tab value="tab3" class="flex items-center gap-2!">
                    <svg data-p-icon="code-branch"></svg>
                    Pull Requests
                </p-tab>
                <p-tab value="tab4" class="flex items-center gap-2!">
                    <svg data-p-icon="cog"></svg>
                    Settings
                </p-tab>
            </p-tablist>
            <p-tabpanels>
                <p-tabpanel value="tab1">
                    <h2 class="text-lg font-bold">Code</h2>
                    <p class="text-surface-500 mt-1">Browse the source files, review the latest commits, and clone the repository to get started.</p>
                </p-tabpanel>
                <p-tabpanel value="tab2">
                    <h2 class="text-lg font-bold">Issues</h2>
                    <p class="text-surface-500 mt-1">Track open bugs, feature requests, and ongoing discussions reported by the community.</p>
                </p-tabpanel>
                <p-tabpanel value="tab3">
                    <h2 class="text-lg font-bold">Pull Requests</h2>
                    <p class="text-surface-500 mt-1">Review proposed changes, leave feedback, and merge contributions into the main branch.</p>
                </p-tabpanel>
                <p-tabpanel value="tab4">
                    <h2 class="text-lg font-bold">Settings</h2>
                    <p class="text-surface-500 mt-1">Manage repository access, configure integrations, and adjust visibility and collaboration rules.</p>
                </p-tabpanel>
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule, Code, InfoCircle, CodeBranch, Cog]
})
export class TabsPreviewDemo {}
```

## Scrollable

Enable scrollable to navigate long tab lists with previous and next buttons.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="0" scrollable>
            <p-tablist>
                @for (tab of scrollableTabs; track tab.value) {
                    <p-tab [value]="tab.value">{{ tab.title }}</p-tab>
                }
            </p-tablist>
            <p-tabpanels>
                @for (tab of scrollableTabs; track tab.value) {
                    <p-tabpanel [value]="tab.value">
                        <h2 class="text-lg font-bold">{{ tab.title }}</h2>
                        <p class="text-surface-500 mt-1">{{ tab.content }}</p>
                    </p-tabpanel>
                }
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsScrollableDemo {
    scrollableTabs = Array.from({ length: 50 }, (_, i) => {
        const start = i % sentences.length;
        const content = Array.from({ length: 4 }, (_, j) => sentences[(start + j) % sentences.length]).join(' ');

        return { title: `Tab ${i + 1}`, value: `${i}`, content };
    });
}
```

## selectonfocus-doc

Set selectOnFocus to activate tabs on focus.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="tab1" [selectOnFocus]="true">
            <p-tablist>
                @for (tab of tabs; track tab.id) {
                    <p-tab [value]="tab.id">{{ tab.title }}</p-tab>
                }
            </p-tablist>
            <p-tabpanels>
                @for (tab of tabs; track tab.id) {
                    <p-tabpanel [value]="tab.id">
                        <h2 class="text-lg font-bold">{{ tab.title }}</h2>
                        <p class="text-surface-500 mt-1">{{ tab.content }}</p>
                    </p-tabpanel>
                }
            </p-tabpanels>
        </p-tabs>
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsSelectonfocusDemo {
    tabs = [
        { id: 'tab1', title: 'Account Info', content: 'Update your personal information such as name, email address, and profile picture.' },
        { id: 'tab2', title: 'Payment', content: 'Manage your subscription plan, view invoices, and update your payment method.' },
        { id: 'tab3', title: 'Preferences', content: 'Customize how the application looks and behaves to match your personal preferences.' }
    ];
}
```

## Tab Menu

A navigation menu is implemented using tabs without the panels where the content of a tab is provided by a route component like router-outlet . For the purpose of this demo, router-outlet is not included.

```typescript
import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

@Component({
    template: `
        <p-tabs value="dashboard">
            <p-tablist>
                @for (tab of tabs; track tab.route) {
                    <p-tab [value]="tab.route" class="flex items-center gap-2! text-inherit">
                        <i [class]="tab.icon"></i>
                        <span>{{ tab.label }}</span>
                    </p-tab>
                }
            </p-tablist>
        </p-tabs>
        <!--<router-outlet></router-outlet>-->
    `,
    standalone: true,
    imports: [TabsModule]
})
export class TabsTabMenuDemo {
    tabs = [
        { route: 'dashboard', label: 'Dashboard', icon: 'pi pi-home' },
        { route: 'transactions', label: 'Transactions', icon: 'pi pi-chart-line' },
        { route: 'products', label: 'Products', icon: 'pi pi-list' },
        { route: 'messages', label: 'Messages', icon: 'pi pi-inbox' }
    ];
}
```

## Tabs

Tabs facilitates seamless switching between different views.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, TabsPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | string \| number \| undefined | undefined | Value of the active tab. |
| scrollable | boolean | false | Scrolling is enabled by default. This property is no longer needed. **(Deprecated)** |
| lazy | boolean | false | When enabled, tabs are not rendered until activation. |
| selectOnFocus | boolean | false | When enabled, the focused tab is activated. |
| showNavigators | boolean | true | Whether to display navigation buttons in container when scrollable is enabled. |
| tabindex | number | 0 | Tabindex of the tab buttons. |
| scrollStrategy | "nearest" \| "center" \| false | 'nearest' | Defines the scrolling behavior when the active tab changes. Use 'nearest' to scroll only if the tab is not fully visible, 'center' to always center the active tab, or false to disable auto-scrolling. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-tabs | Class name of the root element |
| p-tablist | Class name of the wrapper element |
| p-tablist-content | Class name of the content element |
| p-tablist-tab-list | Class name of the tab list element |
| p-tab | Class name of the tab list element |
| p-tablist-active-bar | Class name of the inkbar element |
| p-tablist-nav-button | Class name of the navigation buttons |
| p-tabpanels | Class name of the tab panels wrapper |
| p-tabs-panel | Class name of the tab panel element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| tabs.transition.duration | --p-tabs-transition-duration | Transition duration of root |
| tabs.tablist.border.width | --p-tabs-tablist-border-width | Border width of tablist |
| tabs.tablist.background | --p-tabs-tablist-background | Background of tablist |
| tabs.tablist.border.color | --p-tabs-tablist-border-color | Border color of tablist |
| tabs.tab.background | --p-tabs-tab-background | Background of tab |
| tabs.tab.hover.background | --p-tabs-tab-hover-background | Hover background of tab |
| tabs.tab.active.background | --p-tabs-tab-active-background | Active background of tab |
| tabs.tab.border.width | --p-tabs-tab-border-width | Border width of tab |
| tabs.tab.border.color | --p-tabs-tab-border-color | Border color of tab |
| tabs.tab.hover.border.color | --p-tabs-tab-hover-border-color | Hover border color of tab |
| tabs.tab.active.border.color | --p-tabs-tab-active-border-color | Active border color of tab |
| tabs.tab.color | --p-tabs-tab-color | Color of tab |
| tabs.tab.hover.color | --p-tabs-tab-hover-color | Hover color of tab |
| tabs.tab.active.color | --p-tabs-tab-active-color | Active color of tab |
| tabs.tab.padding | --p-tabs-tab-padding | Padding of tab |
| tabs.tab.font.weight | --p-tabs-tab-font-weight | Font weight of tab |
| tabs.tab.font.size | --p-tabs-tab-font-size | Font size of tab |
| tabs.tab.margin | --p-tabs-tab-margin | Margin of tab |
| tabs.tab.gap | --p-tabs-tab-gap | Gap of tab |
| tabs.tab.focus.ring.width | --p-tabs-tab-focus-ring-width | Focus ring width of tab |
| tabs.tab.focus.ring.style | --p-tabs-tab-focus-ring-style | Focus ring style of tab |
| tabs.tab.focus.ring.color | --p-tabs-tab-focus-ring-color | Focus ring color of tab |
| tabs.tab.focus.ring.offset | --p-tabs-tab-focus-ring-offset | Focus ring offset of tab |
| tabs.tab.focus.ring.shadow | --p-tabs-tab-focus-ring-shadow | Focus ring shadow of tab |
| tabs.tabpanel.background | --p-tabs-tabpanel-background | Background of tabpanel |
| tabs.tabpanel.color | --p-tabs-tabpanel-color | Color of tabpanel |
| tabs.tabpanel.padding | --p-tabs-tabpanel-padding | Padding of tabpanel |
| tabs.tabpanel.focus.ring.width | --p-tabs-tabpanel-focus-ring-width | Focus ring width of tabpanel |
| tabs.tabpanel.focus.ring.style | --p-tabs-tabpanel-focus-ring-style | Focus ring style of tabpanel |
| tabs.tabpanel.focus.ring.color | --p-tabs-tabpanel-focus-ring-color | Focus ring color of tabpanel |
| tabs.tabpanel.focus.ring.offset | --p-tabs-tabpanel-focus-ring-offset | Focus ring offset of tabpanel |
| tabs.tabpanel.focus.ring.shadow | --p-tabs-tabpanel-focus-ring-shadow | Focus ring shadow of tabpanel |
| tabs.nav.button.background | --p-tabs-nav-button-background | Background of nav button |
| tabs.nav.button.color | --p-tabs-nav-button-color | Color of nav button |
| tabs.nav.button.hover.color | --p-tabs-nav-button-hover-color | Hover color of nav button |
| tabs.nav.button.width | --p-tabs-nav-button-width | Width of nav button |
| tabs.nav.button.focus.ring.width | --p-tabs-nav-button-focus-ring-width | Focus ring width of nav button |
| tabs.nav.button.focus.ring.style | --p-tabs-nav-button-focus-ring-style | Focus ring style of nav button |
| tabs.nav.button.focus.ring.color | --p-tabs-nav-button-focus-ring-color | Focus ring color of nav button |
| tabs.nav.button.focus.ring.offset | --p-tabs-nav-button-focus-ring-offset | Focus ring offset of nav button |
| tabs.nav.button.focus.ring.shadow | --p-tabs-nav-button-focus-ring-shadow | Focus ring shadow of nav button |
| tabs.nav.button.shadow | --p-tabs-nav-button-shadow | Shadow of nav button |
| tabs.active.bar.height | --p-tabs-active-bar-height | Height of active bar |
| tabs.active.bar.bottom | --p-tabs-active-bar-bottom | Bottom of active bar |
| tabs.active.bar.background | --p-tabs-active-bar-background | Background of active bar |
