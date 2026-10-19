---
title: "Angular Toast Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/toast.md"
description: "Toast is used to display messages in an overlay."
---

# Angular Toast Component

Toast is used to display messages in an overlay.

## Accessibility

Screen Reader Toast component use alert role that implicitly defines aria-live as "assertive" and aria-atomic as "true". Close element is a button with an aria-label that refers to the aria.close property of the locale API by default. Close Button Keyboard Support Key Function enter Closes the message. space Closes the message.

## Action

An interactive button can be added to the toast content with a custom template, for example to let the user confirm an action.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';
import { Check } from '@primeicons/angular/check';

@Component({
    template: `
        <p-toast key="action">
            <ng-template let-message let-closeFn="closeFn" #message>
                @if (message.severity === 'success') {
                    <svg data-p-icon="check"></svg>
                }
                <div class="p-toast-message-text">
                    <div class="p-toast-summary">{{ message.summary }}</div>
                    <div class="p-toast-detail">{{ message.detail }}</div>
                    @if (message.severity !== 'success') {
                        <button type="button" pButton size="small" class="mt-3 self-start" (click)="enableCamera(closeFn)">Enable camera</button>
                    }
                </div>
            </ng-template>
        </p-toast>
        <div class="flex justify-center">
            <button type="button" pButton outlined (click)="show()">Create toast with action</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule, Check],
    providers: [MessageService]
})
export class ToastActionDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({
            key: 'action',
            summary: 'Allow camera access',
            detail: 'We need access to your camera to scan QR codes.'
        });
    }

    enableCamera(closeFn: (event?: Event) => void) {
        closeFn();
        this.messageService.add({
            key: 'action',
            severity: 'success',
            summary: 'Camera access granted',
            detail: 'You can now scan QR codes.'
        });
    }
}
```

## Basic

Toasts are displayed by calling the add and addAll method provided by the messageService . A single toast is specified by the Message interface that defines various properties such as severity , summary and detail .

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <button type="button" pButton outlined (click)="show()">Create toast</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastBasicDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ summary: 'Successfully completed', detail: 'The task was completed successfully. You can now view the details.' });
    }
}
```

## clear-doc

Clicking the close icon on the toast, removes it manually. Same can also be achieved programmatically using the clear function of the messageService . Calling it without any arguments, removes all displayed messages whereas calling it with a key, removes the messages displayed on a toast having the same key.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast key="myKey" />
        <div class="flex justify-center gap-2">
            <button type="button" pButton (click)="show()">Show</button>
            <button type="button" pButton (click)="clear()" severity="secondary">Clear</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastClearDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ key: 'myKey', severity: 'success', summary: 'Message 1', detail: 'Message Content' });
    }

    clear() {
        this.messageService.clear();
    }
}
```

## Custom

The toast content can be fully customized with a template to render any markup, such as links and action buttons.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast key="custom" [pt]="{ closeButton: { class: 'hidden!' } }">
            <ng-template let-message #message>
                <div class="space-y-2 w-full">
                    <div>
                        <h1 class="text-sm! m-0! font-medium! text-surface-900 dark:text-surface-0 leading-6">
                            Purchase complete!
                            <a class="cursor-pointer underline! hover:opacity-75">View receipt</a>
                        </h1>
                        <p class="text-surface-500 mt-1 text-sm">
                            Your order is being processed. Track all orders or
                            <a class="cursor-pointer underline! hover:opacity-75">learn about returns</a>
                        </p>
                    </div>
                    <div class="flex items-center mt-3 gap-2">
                        <button
                            type="button"
                            (click)="dismiss()"
                            class="font-medium px-2 py-1.5 text-xs rounded-md border border-surface-200 dark:border-surface-700 hover:bg-surface-50 active:bg-surface-100 dark:hover:bg-surface-800 dark:active:bg-surface-700 text-surface-500 dark:text-surface-400"
                        >
                            Dismiss
                        </button>
                        <button
                            type="button"
                            (click)="dismiss()"
                            class="font-medium px-2 py-1.5 text-xs rounded-md border border-indigo-500/25 bg-indigo-50 hover:bg-indigo-100 dark:bg-indigo-950/75 dark:hover:bg-indigo-950 text-indigo-500 dark:text-indigo-400"
                        >
                            Track all orders
                        </button>
                    </div>
                </div>
            </ng-template>
        </p-toast>
        <div class="flex justify-center">
            <button type="button" pButton outlined (click)="show()">Custom toast</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastCustomDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ key: 'custom', life: 6000, summary: 'Purchase complete', styleClass: 'bg-surface-0! dark:bg-surface-900! border-surface!' });
    }

    dismiss() {
        this.messageService.clear('custom');
    }
}
```

## Expanded Mode

Setting mode to expanded always renders the stack expanded; the default stacked mode stacks toasts collapsed and expands them on hover. Auto-dismiss timers still pause while the toasts are hovered.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast key="expanded" mode="expanded" stackVisibleLimit="7" />
        <div class="flex justify-center">
            <button type="button" pButton outlined (click)="show()">Create toast</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastExpandedModeDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ key: 'expanded', summary: 'Successfully completed', detail: 'The task was completed successfully. You can now view the details.' });
    }
}
```

## life-doc

A toast disappears after 3000ms by default, set the life option on either the message or toast to override this.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast [life]="10000" />
        <div class="flex justify-center gap-2">
            <button type="button" pButton (click)="showLife()">Show Life</button>
            <button type="button" pButton (click)="showLifeLong()">Show Life Long</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastLifeDemo {
    private messageService = inject(MessageService);

    showLife() {
        this.messageService.add({ severity: 'info', summary: 'Life', detail: 'I show for 10000ms' });
    }

    showLifeLong() {
        this.messageService.add({ severity: 'info', summary: 'Life', detail: 'I show for 20000ms', life: 20000 });
    }
}
```

## Position

Location of the toast is customized with the position property. Valid values are top-left , top-center , top-right , bottom-left , bottom-center , bottom-right and center .

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast position="top-left" key="top-left" />
        <p-toast position="top-center" key="top-center" />
        <p-toast position="top-right" key="top-right" />
        <p-toast position="bottom-left" key="bottom-left" />
        <p-toast position="bottom-center" key="bottom-center" />
        <p-toast position="bottom-right" key="bottom-right" />
        <p-toast position="center" key="center" />
        <div class="flex justify-center">
            <div class="flex flex-wrap gap-2">
                <button type="button" pButton pRipple outlined (click)="createToast('top-left')">Top Left</button>
                <button type="button" pButton pRipple outlined (click)="createToast('top-center')">Top Center</button>
                <button type="button" pButton pRipple outlined (click)="createToast('top-right')">Top Right</button>
                <button type="button" pButton pRipple outlined (click)="createToast('bottom-left')">Bottom Left</button>
                <button type="button" pButton pRipple outlined (click)="createToast('bottom-center')">Bottom Center</button>
                <button type="button" pButton pRipple outlined (click)="createToast('bottom-right')">Bottom Right</button>
                <button type="button" pButton pRipple outlined (click)="createToast('center')">Center</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule, RippleModule],
    providers: [MessageService]
})
export class ToastPositionDemo {
    private messageService = inject(MessageService);

    createToast(key: string) {
        this.messageService.add({
            summary: 'Successfully completed',
            detail: 'The task was completed successfully. You can now view the details.',
            key
        });
    }
}
```

## preview-doc

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <button type="button" pButton (click)="show()">Create toast</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastPreviewDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ summary: 'Successfully completed', detail: 'The task was completed successfully. You can now view the details.' });
    }
}
```

## Promise

A sticky loading toast can be shown while an async task runs, then removed and replaced with a success or error toast based on the result.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <div class="flex flex-wrap items-center justify-center gap-4">
            <p-toast key="promise" />
            <button type="button" pButton outlined (click)="runPromise()">Run promise</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastPromiseDemo {
    private messageService = inject(MessageService);

    fakeApiCall() {
        return new Promise<string>((resolve, reject) => {
            setTimeout(() => {
                if (Math.random() < 0.5) {
                    resolve('Success!');
                } else {
                    reject(new Error('Something went wrong'));
                }
            }, 2000);
        });
    }

    runPromise() {
        this.messageService.add({
            severity: 'secondary',
            summary: 'Please wait...',
            detail: 'Your request is being processed. This may take a moment.',
            icon: 'pi pi-spinner pi-spin',
            key: 'promise',
            sticky: true
        });

        this.fakeApiCall()
            .then((result) => {
                this.messageService.clear('promise');
                setTimeout(() => {
                    this.messageService.add({ severity: 'success', summary: 'Success!', detail: `${result}. Everything went smoothly. Thank you for your patience.`, key: 'promise', life: 3000 });
                }, 300);
            })
            .catch((error) => {
                this.messageService.clear('promise');
                setTimeout(() => {
                    this.messageService.add({ severity: 'error', summary: 'Oops, something went wrong', detail: `Error: ${error.message}. Please try again or contact support if the problem persists.`, key: 'promise', life: 3000 });
                }, 300);
            });
    }
}
```

## Severity

The severity option specifies the type of the message. There are four types of messages: success , info , warn and error . The severity of the message is used to display the icon and the color of the toast.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex flex-wrap justify-center gap-2">
            <button type="button" pButton pRipple outlined (click)="showInfo()" severity="info">Info</button>
            <button type="button" pButton pRipple outlined (click)="showSuccess()" severity="success">Success</button>
            <button type="button" pButton pRipple outlined (click)="showWarn()" severity="warn">Warn</button>
            <button type="button" pButton pRipple outlined (click)="showError()" severity="danger">Error</button>
            <button type="button" pButton pRipple outlined (click)="showSecondary()" severity="secondary">Secondary</button>
            <button type="button" pButton pRipple outlined (click)="showContrast()" severity="contrast">Contrast</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule, RippleModule],
    providers: [MessageService]
})
export class ToastSeverityDemo {
    private messageService = inject(MessageService);

    showInfo() {
        this.messageService.add({ severity: 'info', summary: 'Heads up', detail: 'There’s something you might want to check.' });
    }

    showSuccess() {
        this.messageService.add({ severity: 'success', summary: 'Saved successfully', detail: 'Your changes have been saved.' });
    }

    showWarn() {
        this.messageService.add({ severity: 'warn', summary: 'Check this', detail: 'Some fields may need your attention.' });
    }

    showError() {
        this.messageService.add({ severity: 'error', summary: 'Something went wrong', detail: 'We couldn’t complete the action. Please try again.' });
    }

    showSecondary() {
        this.messageService.add({ severity: 'secondary', summary: 'For your information', detail: 'This is a secondary toast message.' });
    }

    showContrast() {
        this.messageService.add({ severity: 'contrast', summary: 'High contrast', detail: 'This is a contrast toast message.' });
    }
}
```

## Sticky

A toast disappears after the time defined by the life option, set sticky option true on the message to override this and not hide the toast automatically.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <div class="flex flex-wrap gap-2">
                <button pButton pRipple outlined (click)="show()">Create toast</button>
                <button pButton pRipple outlined (click)="clear()" severity="secondary">Dismiss toast</button>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule, RippleModule],
    providers: [MessageService]
})
export class ToastStickyDemo {
    private messageService = inject(MessageService);

    show() {
        this.messageService.add({ severity: 'info', summary: 'Sticky toast', detail: 'This toast stays until you dismiss it manually.', sticky: true });
    }

    clear() {
        this.messageService.clear();
    }
}
```

## target-doc

A page may have multiple toast components, in case you'd like to target a specific message to a particular toast, use the key property so that toast and the message can match.

```typescript
import { Component, inject } from '@angular/core';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast key="toast1" />
        <p-toast key="toast2" />
        <div class="flex justify-center gap-2">
            <button pButton (click)="showToast1()">Show Success</button>
            <button pButton (click)="showToast2()" severity="warn">Show Warning</button>
        </div>
    `,
    standalone: true,
    imports: [ToastModule, ButtonModule],
    providers: [MessageService]
})
export class ToastTargetDemo {
    private messageService = inject(MessageService);

    showToast1() {
        this.messageService.clear();
        this.messageService.add({ key: 'toast1', severity: 'success', summary: 'Success', detail: 'key: toast1' });
    }

    showToast2() {
        this.messageService.clear();
        this.messageService.add({ key: 'toast2', severity: 'warn', summary: 'Warning', detail: 'key: toast2' });
    }
}
```

## Toast

Toast is used to display messages in an overlay.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ToastPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| key | string \| undefined | - | Key of the message in case message is targeted to a specific toast component. |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| life | number | - | The default time to display messages for in milliseconds. |
| position | "top-left" \| "top-center" \| "top-right" \| "bottom-left" \| "bottom-center" \| "bottom-right" \| "center" | - | Position of the toast in viewport. |
| mode | "stacked" \| "expanded" | 'stacked' | Layout mode of the toast.  `stacked`  collapses toasts into a stack that expands on hover/focus;  `expanded`  always renders every toast expanded. |
| stackGap | number | - | Gap between stacked toast items in pixels. |
| stackVisibleLimit | number | - | Maximum number of visible toasts in the stack. |
| preventOpenDuplicates | boolean | - | It does not add the new message if there is already a toast displayed with the same content |
| preventDuplicates | boolean | - | Displays only once a message with the same content. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| breakpoints | ToastBreakpoints \| undefined | - | Object literal to define styles per screen size. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onClose | event: ToastCloseEvent | Callback to invoke when a message is closed. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| message | TemplateRef<ToastMessageTemplateContext> \| undefined | Custom message template. |
| headless | TemplateRef<ToastHeadlessTemplateContext> \| undefined | Custom headless template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| message | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the message's DOM element. |
| messageContent | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the message content's DOM element. |
| messageIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the message icon's DOM element. |
| messageText | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the message text's DOM element. |
| summary | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the summary's DOM element. |
| detail | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the detail's DOM element. |
| closeButton | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the close button's DOM element. |
| closeIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the close icon's DOM element. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-toast | Class name of the root element |
| p-toast-message | Class name of the message element |
| p-toast-message-content | Class name of the message content element |
| p-toast-message-icon | Class name of the message icon element |
| p-toast-message-text | Class name of the message text element |
| p-toast-summary | Class name of the summary element |
| p-toast-detail | Class name of the detail element |
| p-toast-close-button | Class name of the close button element |
| p-toast-close-icon | Class name of the close icon element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| toast.width | --p-toast-width | Width of root |
| toast.border.radius | --p-toast-border-radius | Border radius of root |
| toast.border.width | --p-toast-border-width | Border width of root |
| toast.transition.duration | --p-toast-transition-duration | Transition duration of root |
| toast.blur | --p-toast-blur | Used to pass tokens of the blur section |
| toast.focus.ring.width | --p-toast-focus-ring-width | Focus ring width of root |
| toast.focus.ring.style | --p-toast-focus-ring-style | Focus ring style of root |
| toast.focus.ring.color | --p-toast-focus-ring-color | Focus ring color of root |
| toast.focus.ring.offset | --p-toast-focus-ring-offset | Focus ring offset of root |
| toast.focus.ring.shadow | --p-toast-focus-ring-shadow | Focus ring shadow of root |
| toast.icon.size | --p-toast-icon-size | Size of icon |
| toast.icon.margin | --p-toast-icon-margin | Margin of icon |
| toast.content.padding | --p-toast-content-padding | Padding of content |
| toast.content.gap | --p-toast-content-gap | Gap of content |
| toast.text.gap | --p-toast-text-gap | Gap of text |
| toast.summary.font.weight | --p-toast-summary-font-weight | Font weight of summary |
| toast.summary.font.size | --p-toast-summary-font-size | Font size of summary |
| toast.detail.font.weight | --p-toast-detail-font-weight | Font weight of detail |
| toast.detail.font.size | --p-toast-detail-font-size | Font size of detail |
| toast.close.button.width | --p-toast-close-button-width | Width of close button |
| toast.close.button.height | --p-toast-close-button-height | Height of close button |
| toast.close.button.border.radius | --p-toast-close-button-border-radius | Border radius of close button |
| toast.close.button.focus.ring.width | --p-toast-close-button-focus-ring-width | Focus ring width of close button |
| toast.close.button.focus.ring.style | --p-toast-close-button-focus-ring-style | Focus ring style of close button |
| toast.close.button.focus.ring.offset | --p-toast-close-button-focus-ring-offset | Focus ring offset of close button |
| toast.close.icon.size | --p-toast-close-icon-size | Size of close icon |
| toast.normal.background | --p-toast-normal-background | Background of normal |
| toast.normal.border.color | --p-toast-normal-border-color | Border color of normal |
| toast.normal.color | --p-toast-normal-color | Color of normal |
| toast.normal.detail.color | --p-toast-normal-detail-color | Detail color of normal |
| toast.normal.shadow | --p-toast-normal-shadow | Shadow of normal |
| toast.normal.close.button.hover.background | --p-toast-normal-close-button-hover-background | Close button hover background of normal |
| toast.normal.close.button.focus.ring.color | --p-toast-normal-close-button-focus-ring-color | Close button focus ring color of normal |
| toast.normal.close.button.focus.ring.shadow | --p-toast-normal-close-button-focus-ring-shadow | Close button focus ring shadow of normal |
| toast.info.background | --p-toast-info-background | Background of info |
| toast.info.border.color | --p-toast-info-border-color | Border color of info |
| toast.info.color | --p-toast-info-color | Color of info |
| toast.info.detail.color | --p-toast-info-detail-color | Detail color of info |
| toast.info.shadow | --p-toast-info-shadow | Shadow of info |
| toast.info.close.button.hover.background | --p-toast-info-close-button-hover-background | Close button hover background of info |
| toast.info.close.button.focus.ring.color | --p-toast-info-close-button-focus-ring-color | Close button focus ring color of info |
| toast.info.close.button.focus.ring.shadow | --p-toast-info-close-button-focus-ring-shadow | Close button focus ring shadow of info |
| toast.success.background | --p-toast-success-background | Background of success |
| toast.success.border.color | --p-toast-success-border-color | Border color of success |
| toast.success.color | --p-toast-success-color | Color of success |
| toast.success.detail.color | --p-toast-success-detail-color | Detail color of success |
| toast.success.shadow | --p-toast-success-shadow | Shadow of success |
| toast.success.close.button.hover.background | --p-toast-success-close-button-hover-background | Close button hover background of success |
| toast.success.close.button.focus.ring.color | --p-toast-success-close-button-focus-ring-color | Close button focus ring color of success |
| toast.success.close.button.focus.ring.shadow | --p-toast-success-close-button-focus-ring-shadow | Close button focus ring shadow of success |
| toast.warn.background | --p-toast-warn-background | Background of warn |
| toast.warn.border.color | --p-toast-warn-border-color | Border color of warn |
| toast.warn.color | --p-toast-warn-color | Color of warn |
| toast.warn.detail.color | --p-toast-warn-detail-color | Detail color of warn |
| toast.warn.shadow | --p-toast-warn-shadow | Shadow of warn |
| toast.warn.close.button.hover.background | --p-toast-warn-close-button-hover-background | Close button hover background of warn |
| toast.warn.close.button.focus.ring.color | --p-toast-warn-close-button-focus-ring-color | Close button focus ring color of warn |
| toast.warn.close.button.focus.ring.shadow | --p-toast-warn-close-button-focus-ring-shadow | Close button focus ring shadow of warn |
| toast.error.background | --p-toast-error-background | Background of error |
| toast.error.border.color | --p-toast-error-border-color | Border color of error |
| toast.error.color | --p-toast-error-color | Color of error |
| toast.error.detail.color | --p-toast-error-detail-color | Detail color of error |
| toast.error.shadow | --p-toast-error-shadow | Shadow of error |
| toast.error.close.button.hover.background | --p-toast-error-close-button-hover-background | Close button hover background of error |
| toast.error.close.button.focus.ring.color | --p-toast-error-close-button-focus-ring-color | Close button focus ring color of error |
| toast.error.close.button.focus.ring.shadow | --p-toast-error-close-button-focus-ring-shadow | Close button focus ring shadow of error |
| toast.secondary.background | --p-toast-secondary-background | Background of secondary |
| toast.secondary.border.color | --p-toast-secondary-border-color | Border color of secondary |
| toast.secondary.color | --p-toast-secondary-color | Color of secondary |
| toast.secondary.detail.color | --p-toast-secondary-detail-color | Detail color of secondary |
| toast.secondary.shadow | --p-toast-secondary-shadow | Shadow of secondary |
| toast.secondary.close.button.hover.background | --p-toast-secondary-close-button-hover-background | Close button hover background of secondary |
| toast.secondary.close.button.focus.ring.color | --p-toast-secondary-close-button-focus-ring-color | Close button focus ring color of secondary |
| toast.secondary.close.button.focus.ring.shadow | --p-toast-secondary-close-button-focus-ring-shadow | Close button focus ring shadow of secondary |
| toast.contrast.background | --p-toast-contrast-background | Background of contrast |
| toast.contrast.border.color | --p-toast-contrast-border-color | Border color of contrast |
| toast.contrast.color | --p-toast-contrast-color | Color of contrast |
| toast.contrast.detail.color | --p-toast-contrast-detail-color | Detail color of contrast |
| toast.contrast.shadow | --p-toast-contrast-shadow | Shadow of contrast |
| toast.contrast.close.button.hover.background | --p-toast-contrast-close-button-hover-background | Close button hover background of contrast |
| toast.contrast.close.button.focus.ring.color | --p-toast-contrast-close-button-focus-ring-color | Close button focus ring color of contrast |
| toast.contrast.close.button.focus.ring.shadow | --p-toast-contrast-close-button-focus-ring-shadow | Close button focus ring shadow of contrast |
