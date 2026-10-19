---
title: "Angular ConfirmDialog Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/confirmdialog.md"
description: "ConfirmDialog is backed by a service utilizing Observables to display confirmation windows easily that can be shared by multiple actions on the same component."
---

# Angular ConfirmDialog Component

ConfirmDialog is backed by a service utilizing Observables to display confirmation windows easily that can be shared by multiple actions on the same component.

## Accessibility

Screen Reader ConfirmDialog component uses alertdialog role along with aria-labelledby referring to the header element however any attribute is passed to the root element so you may use aria-labelledby to override this default behavior. In addition aria-modal is added since focus is kept within the popup. It is recommended to use a trigger component that can be accessed with keyboard such as a button, if not adding tabIndex would be necessary. When confirm function is used and a trigger is passed as a parameter, ConfirmDialog adds aria-expanded state attribute and aria-controls to the trigger so that the relation between the trigger and the popup is defined. If the dialog is controlled with the visible property aria-expanded and aria-controls need to be handled explicitly. Overlay Keyboard Support Key Function tab Moves focus to the next the focusable element within the popup. shift + tab Moves focus to the previous the focusable element within the popup. escape Closes the popup and moves focus to the trigger. Buttons Keyboard Support Key Function enter Triggers the action, closes the popup and moves focus to the trigger. space Triggers the action, closes the popup and moves focus to the trigger.

## Basic

ConfirmDialog is defined using p-confirmdialog tag and an instance of ConfirmationService is required to display it bycalling confirm method.

```typescript
import { Component, inject } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { MessageService, ConfirmationService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center gap-2">
            <button type="button" pButton (click)="confirm1($event)" [outlined]="true">Save</button>
            <button type="button" pButton (click)="confirm2($event)" severity="danger" [outlined]="true">Delete</button>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule],
    providers: [ConfirmationService, MessageService]
})
export class ConfirmDialogBasicDemo {
    private confirmationService = inject(ConfirmationService);

    private messageService = inject(MessageService);

    confirm1(event: Event) {
        this.confirmationService.confirm({
            target: event.target as EventTarget,
            message: 'Are you sure that you want to proceed?',
            header: 'Confirmation',
            closable: true,
            closeOnEscape: true,
            icon: 'pi pi-exclamation-triangle',
            rejectButtonProps: {
                label: 'Cancel',
                severity: 'secondary',
                outlined: true
            },
            acceptButtonProps: {
                label: 'Save'
            },
            accept: () => {
                this.messageService.add({ severity: 'info', summary: 'Confirmed', detail: 'You have accepted' });
            },
            reject: () => {
                this.messageService.add({
                    severity: 'error',
                    summary: 'Rejected',
                    detail: 'You have rejected',
                    life: 3000
                });
            }
        });
    }

    confirm2(event: Event) {
        this.confirmationService.confirm({
            target: event.target as EventTarget,
            message: 'Do you want to delete this record?',
            header: 'Danger Zone',
            icon: 'pi pi-info-circle',
            rejectLabel: 'Cancel',
            rejectButtonProps: {
                label: 'Cancel',
                severity: 'secondary',
                outlined: true
            },
            acceptButtonProps: {
                label: 'Delete',
                severity: 'danger'
            },

            accept: () => {
                this.messageService.add({ severity: 'info', summary: 'Confirmed', detail: 'Record deleted' });
            },
            reject: () => {
                this.messageService.add({ severity: 'error', summary: 'Rejected', detail: 'You have rejected' });
            }
        });
    }
}
```

## Headless

Headless mode allows you to customize the entire user interface instead of the default elements.

```typescript
import { Component, inject } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { MessageService, ConfirmationService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="confirm()">Save</button>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule],
    providers: [ConfirmationService, MessageService]
})
export class ConfirmDialogHeadlessDemo {
    private confirmationService = inject(ConfirmationService);

    private messageService = inject(MessageService);

    confirm() {
        this.confirmationService.confirm({
            header: 'Are you sure?',
            message: 'Please confirm to proceed.',
            accept: () => {
                this.messageService.add({ severity: 'info', summary: 'Confirmed', detail: 'You have accepted' });
            },
            reject: () => {
                this.messageService.add({ severity: 'info', summary: 'Rejected', detail: 'You have rejected' });
            }
        });
    }
}
```

## Position

The position property of the confirm options is used to display a Dialog at all edges and corners of the screen.

```typescript
import { Component, inject } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { MessageService, ConfirmationService } from 'primeng/api';
import { Dialog } from 'primeng/dialog';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2 mb-4">
            <button type="button" pButton (click)="confirmPosition('left')" severity="secondary" class="min-w-40">Left</button>
            <button type="button" pButton (click)="confirmPosition('right')" severity="secondary" class="min-w-40">Right</button>
        </div>
        <div class="flex flex-wrap justify-center gap-2 mb-4">
            <button type="button" pButton (click)="confirmPosition('topleft')" severity="secondary" class="min-w-40">TopLeft</button>
            <button type="button" pButton (click)="confirmPosition('top')" severity="secondary" class="min-w-40">Top</button>
            <button type="button" pButton (click)="confirmPosition('topright')" severity="secondary" class="min-w-40">TopRight</button>
        </div>
        <div class="flex flex-wrap justify-center gap-2">
            <button type="button" pButton (click)="confirmPosition('bottomleft')" severity="secondary" class="min-w-40">BottomLeft</button>
            <button type="button" pButton (click)="confirmPosition('bottom')" severity="secondary" class="min-w-40">Bottom</button>
            <button type="button" pButton (click)="confirmPosition('bottomright')" severity="secondary" class="min-w-40">BottomRight</button>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule],
    providers: [ConfirmationService, MessageService]
})
export class ConfirmDialogPositionDemo {
    private confirmationService = inject(ConfirmationService);

    private messageService = inject(MessageService);

    confirmPosition(position: 'left' | 'right' | 'top' | 'bottom' | 'center' | 'topleft' | 'topright' | 'bottomleft' | 'bottomright') {
        this.position = position;

        this.confirmationService.confirm({
            message: 'Are you sure you want to proceed?',
            header: 'Confirmation',
            icon: 'pi pi-info-circle',
            rejectButtonStyleClass: 'p-button-text',
            rejectButtonProps: {
                label: 'Cancel',
                severity: 'secondary',
                text: true
            },
            acceptButtonProps: {
                label: 'Save',
                text: true
            },
            accept: () => {
                this.messageService.add({ severity: 'info', summary: 'Confirmed', detail: 'Request submitted' });
            },
            reject: () => {
                this.messageService.add({
                    severity: 'error',
                    summary: 'Rejected',
                    detail: 'Process incomplete',
                    life: 3000
                });
            },
            key: 'positionDialog'
        });
    }
}
```

## Template

Properties of the dialog are defined in two ways, message , icon , header properties can either be defined using confirm method or declaratively on p-confirmDialog ng-template by header , message , icon and footer templates. If these values are unlikely to change then declarative approach would be useful, still properties defined in a ng-template can be overridden with confirm method call. In addition, buttons at footer section can be customized by passing your own UI, important note to make confirmation work with a custom UI is defining a local ng-template variable for the dialog and assign accept()-reject() methods to your own buttons.

```typescript
import { Component, inject } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { MessageService, ConfirmationService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="confirm()">Save</button>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule],
    providers: [ConfirmationService, MessageService]
})
export class ConfirmDialogTemplateDemo {
    private confirmationService = inject(ConfirmationService);

    private messageService = inject(MessageService);

    confirm() {
        this.confirmationService.confirm({
            header: 'Confirmation',
            message: 'Please confirm to proceed moving forward.',
            icon: 'pi pi-exclamation-circle',
            rejectButtonProps: {
                label: 'Cancel',
                icon: 'pi pi-times',
                variant: 'outlined',
                size: 'small'
            },
            acceptButtonProps: {
                label: 'Save',
                icon: 'pi pi-check',
                size: 'small'
            },
            accept: () => {
                this.messageService.add({ severity: 'info', summary: 'Confirmed', detail: 'You have accepted', life: 3000 });
            },
            reject: () => {
                this.messageService.add({ severity: 'error', summary: 'Rejected', detail: 'You have rejected', life: 3000 });
            }
        });
    }
}
```

## Confirm Dialog

ConfirmDialog uses a Dialog UI that is integrated with the Confirmation API.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ConfirmDialogPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| header | string \| undefined | - | Title text of the dialog. |
| icon | string \| undefined | - | Icon to display next to message. |
| message | string \| undefined | - | Message of the confirmation. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the element. |
| styleClass | string \| undefined | - | Class of the element. |
| maskStyleClass | string \| undefined | - | Specify the CSS class(es) for styling the mask element |
| acceptIcon | string \| undefined | - | Icon of the accept button. |
| acceptLabel | string \| undefined | - | Label of the accept button. |
| closeAriaLabel | string \| undefined | - | Defines a string that labels the close button for accessibility. |
| acceptAriaLabel | string \| undefined | - | Defines a string that labels the accept button for accessibility. |
| acceptVisible | boolean | - | Visibility of the accept button. |
| rejectIcon | string \| undefined | - | Icon of the reject button. |
| rejectLabel | string \| undefined | - | Label of the reject button. |
| rejectAriaLabel | string \| undefined | - | Defines a string that labels the reject button for accessibility. |
| rejectVisible | boolean | - | Visibility of the reject button. |
| acceptButtonStyleClass | string \| undefined | - | Style class of the accept button. |
| rejectButtonStyleClass | string \| undefined | - | Style class of the reject button. |
| closeOnEscape | boolean | - | Specifies if pressing escape key should hide the dialog. |
| dismissableMask | boolean \| undefined | - | Specifies if clicking the modal background should hide the dialog. |
| blockScroll | boolean | - | Determines whether scrolling behavior should be blocked within the component. |
| rtl | boolean | - | When enabled dialog is displayed in RTL direction. |
| closable | boolean | - | Adds a close icon to the header to hide the dialog. |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'body' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| key | string \| undefined | - | Optional key to match the key of confirm object, necessary to use when component tree has multiple confirm dialogs. |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| maskMotionOptions | MotionOptions \| undefined | - | The motion options for the mask. |
| focusTrap | boolean | - | When enabled, can only focus on elements inside the confirm dialog. |
| defaultFocus | "accept" \| "reject" \| "close" \| "none" | - | Element to receive the focus when the dialog gets visible. |
| breakpoints | Record<string, string> \| undefined | - | Object literal to define widths per screen size. |
| modal | boolean | - | Defines if background should be blocked when dialog is displayed. |
| visible | boolean | - | Current visible state as a boolean. |
| position | "center" \| "top" \| "bottom" \| "left" \| "right" \| "topleft" \| "topright" \| "bottomleft" \| "bottomright" | - | Allows getting the position of the component. |
| draggable | boolean | - | Enables dragging to change the position using header. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onHide | event: ConfirmEventType \| undefined | Callback to invoke when dialog is hidden. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<void> \| undefined | Custom header template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| rejecticon | TemplateRef<void> \| undefined | Custom reject icon template. |
| accepticon | TemplateRef<void> \| undefined | Custom accept icon template. |
| message | TemplateRef<ConfirmDialogMessageTemplateContext> \| undefined | Custom message template. |
| icon | TemplateRef<void> \| undefined | Custom icon template. |
| headless | TemplateRef<ConfirmDialogHeadlessTemplateContext> \| undefined | Custom headless template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| mask | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the mask's DOM element. |
| icon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the icon's DOM element. |
| message | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the message's DOM element. |
| resizeHandle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the resize handle's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| title | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the title's DOM element. |
| headerActions | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header actions' DOM element. |
| pcCloseButton | ButtonPassThrough | Used to pass attributes to the close Button component. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |
| pcAcceptButton | ButtonPassThrough | Used to pass attributes to the accept Button component. |
| pcRejectButton | ButtonPassThrough | Used to pass attributes to the reject Button component. |
| motion | MotionOptions | Used to pass options to the motion component/directive. |
| maskMotion | MotionOptions | Used to pass motion options for the mask animation. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-confirmdialog | Class name of the root element |
| p-confirmdialog-icon | Class name of the icon element |
| p-confirmdialog-message | Class name of the message element |
| p-confirmdialog-reject-button | Class name of the reject button element |
| p-confirmdialog-accept-button | Class name of the accept button element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| confirmdialog.icon.size | --p-confirmdialog-icon-size | Size of icon |
| confirmdialog.icon.color | --p-confirmdialog-icon-color | Color of icon |
| confirmdialog.content.gap | --p-confirmdialog-content-gap | Gap of content |
| confirmdialog.message.color | --p-confirmdialog-message-color | Color of message |
| confirmdialog.message.font.weight | --p-confirmdialog-message-font-weight | Font weight of message |
| confirmdialog.message.font.size | --p-confirmdialog-message-font-size | Font size of message |
