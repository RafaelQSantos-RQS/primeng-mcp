---
title: "Angular BlockUI Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/blockui.md"
description: "BlockUI can either block other components or the whole page."
---

# Angular BlockUI Component

BlockUI can either block other components or the whole page.

## Accessibility

Screen Reader BlockUI manages aria-busy state attribute when the UI gets blocked and unblocked. Any valid attribute is passed to the root element so additional attributes like role and aria-live can be used to define live regions. Keyboard Support Component does not include any interactive elements.

## Basic

The element to block should be placed as a child of BlockUI and blocked property is required to control the state.

```typescript
import { Component } from '@angular/core';
import { BlockUIModule } from 'primeng/blockui';
import { PanelModule } from 'primeng/panel';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <button type="button" pButton (click)="blockedPanel = true" class="me-2" severity="secondary">Block</button>
        <button type="button" pButton (click)="blockedPanel = false" severity="secondary">Unblock</button>
        <p-blockui [target]="pnl" [blocked]="blockedPanel" />
        <p-panel #pnl header="Header" class="mt-5">
            <p class="m-0 text-sm">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </p>
        </p-panel>
    `,
    standalone: true,
    imports: [BlockUIModule, PanelModule, ButtonModule]
})
export class BlockUIBasicDemo {
    blockedPanel: boolean = false;
}
```

## Document

If the target element is not specified, BlockUI blocks the document by default.

```typescript
import { Component, inject, ChangeDetectorRef } from '@angular/core';
import { BlockUIModule } from 'primeng/blockui';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <p-blockui [blocked]="blockedDocument" />
            <button type="button" pButton (click)="blockDocument()">Block</button>
        </div>
    `,
    standalone: true,
    imports: [BlockUIModule, ButtonModule]
})
export class BlockUIDocumentDemo {
    private cd = inject(ChangeDetectorRef);

    blockedDocument: boolean = false;

    blockDocument() {
        this.blockedDocument = true;
        setTimeout(() => {
            this.blockedDocument = false;
            this.cd.markForCheck();
        }, 3000);
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { BlockUIModule } from 'primeng/blockui';
import { CardModule } from 'primeng/card';
import { ToggleButtonModule } from 'primeng/togglebutton';
import { Lock } from '@primeicons/angular/lock';
import { LockOpen } from '@primeicons/angular/lock-open';

@Component({
    template: `
        <div class="max-w-sm mx-auto flex flex-col gap-4">
            <p-togglebutton [(ngModel)]="blocked" onLabel="Unblock" offLabel="Block" class="w-full">
                <ng-template #icon let-checked>
                    @if (checked) {
                        <svg data-p-icon="lock" />
                    } @else {
                        <svg data-p-icon="lock-open" />
                    }
                </ng-template>
            </p-togglebutton>
            <p-blockui [target]="card" [blocked]="blocked" />
            <p-card #card header="Account Settings" subheader="Manage your workspace preferences">
                <p class="m-0 text-sm">Update your profile, configure notifications and control privacy. Block the panel to prevent changes while your settings are being saved.</p>
            </p-card>
        </div>
    `,
    standalone: true,
    imports: [BlockUIModule, CardModule, ToggleButtonModule, FormsModule, Lock, LockOpen]
})
export class BlockUIPreviewDemo {
    blocked: boolean = false;
}
```

## Block U I

BlockUI can either block other components or the whole page.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, BlockUIPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| target | any | - | Name of the local ng-template variable referring to another component. |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| blocked | boolean | - | Current blocked state as a boolean. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| content | TemplateRef<any> \| undefined | Template of the content. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-blockui | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| blockui.border.radius | --p-blockui-border-radius | Border radius of root |
