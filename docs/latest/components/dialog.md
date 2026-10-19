---
title: "Angular Dialog Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/dialog.md"
description: "Dialog is a container to display content in an overlay window."
---

# Angular Dialog Component

Dialog is a container to display content in an overlay window.

## Accessibility

Screen Reader Dialog component uses dialog role along with aria-labelledby referring to the header element however any attribute is passed to the root element so you may use aria-labelledby to override this default behavior. In addition aria-modal is added since focus is kept within the popup. It is recommended to use a trigger component that can be accessed with keyboard such as a button, if not adding tabIndex would be necessary. Trigger element also requires aria-expanded and aria-controls to be handled explicitly. Close element is a button with an aria-label that refers to the aria.close property of the locale API by default, you may use closeButtonProps to customize the element and override the default aria-label . Maximize element is a button with an aria-label that refers to the aria.maximizeLabel and aria.minimizeLabel property of the locale API. It cannot be customized using the maximizeButtonProps . Overlay Keyboard Support Key Function tab Moves focus to the next the focusable element within the dialog. shift + tab Moves focus to the previous the focusable element within the dialog. escape Closes the dialog if closeOnEscape is true. Close Button Keyboard Support Key Function enter Closes the dialog. space Closes the dialog.

## Basic

Dialog is used as a container and visibility is controlled with a binding to visible property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">Edit Profile</button>
            <p-dialog [(visible)]="visible" [modal]="true" header="Edit Profile" [style]="{ width: '24rem' }">
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-1.5">
                        <label pLabel for="name">Name</label>
                        <input pInputText id="name" [(ngModel)]="name" />
                    </div>
                    <div class="flex flex-col gap-1.5">
                        <label pLabel for="email">Email</label>
                        <input pInputText id="email" [(ngModel)]="email" />
                    </div>
                </div>
                <ng-template #footer>
                    <button type="button" pButton severity="secondary" variant="outlined" (click)="visible = false">Cancel</button>
                    <button type="button" pButton (click)="visible = false">Save</button>
                </ng-template>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule, LabelModule, FormsModule]
})
export class DialogBasicDemo {
    visible: boolean = false;

    name: string = 'Amanda Miller';

    email: string = 'amanda@example.com';
}
```

## Confirmation

A confirmation dialog can be built by disabling the header close button with closable and providing explicit actions in the content.

```typescript
import { Component } from '@angular/core';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton severity="danger" (click)="visible = true">Delete Account</button>
            <p-dialog [(visible)]="visible" [modal]="true" [closable]="false" header="Delete your account." [style]="{ width: '26rem' }">
                <div class="flex flex-col gap-4">
                    <p class="text-surface-500 dark:text-surface-400 text-sm mt-0 mb-0">This action cannot be undone. All of your data, including projects, files, and settings will be permanently removed.</p>
                    <div class="flex justify-end gap-2">
                        <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                        <button type="button" pButton severity="danger" (click)="visible = false">Delete</button>
                    </div>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule]
})
export class DialogConfirmationDemo {
    visible: boolean = false;
}
```

## Draggable

Enabling the draggable property lets the user reposition the Dialog by dragging it from its header.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Dialog, DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { TextareaModule } from 'primeng/textarea';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">Open Note</button>
            <p-dialog [(visible)]="visible" [draggable]="true" [modal]="true" header="Quick Note" [style]="{ width: '24rem' }">
                <div class="flex flex-col gap-4">
                    <p class="text-sm text-surface-500 dark:text-surface-400 mt-0 mb-0">Drag this dialog by its header to reposition it anywhere on the screen.</p>
                    <textarea pTextarea [(ngModel)]="note" class="w-full h-32 resize-none" placeholder="Type your note here..."></textarea>
                </div>
                <ng-template #footer>
                    <button type="button" pButton severity="secondary" (click)="visible = false">Discard</button>
                    <button type="button" pButton (click)="visible = false">Save Note</button>
                </ng-template>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, TextareaModule, FormsModule]
})
export class DialogDraggableDemo {
    visible: boolean = false;

    note: string = '';
}
```

## Full Screen

A full screen Dialog can be achieved by sizing the component to the viewport with utility classes such as w-screen and h-screen .

```typescript
import { Component } from '@angular/core';
import { Dialog, DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { Photo } from '@/domain/photo';
import { Image } from '@primeicons/angular/image';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">View Gallery</button>
            <p-dialog [(visible)]="visible" [modal]="true" header="Photo Gallery" styleClass="w-screen! h-screen! max-h-screen! rounded-none!">
                <div class="flex flex-col gap-4">
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3">
                        @for (i of photos; track i) {
                            <div class="aspect-square rounded-lg bg-surface-100 dark:bg-surface-800 flex items-center justify-center">
                                <svg data-p-icon="image" class="text-2xl text-surface-400" />
                            </div>
                        }
                    </div>
                    <p class="text-sm text-surface-500 dark:text-surface-400 mt-0 mb-0">Showing 18 of 64 photos</p>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, Image]
})
export class DialogFullScreenDemo {
    visible: boolean = false;

    photos = Array.from({ length: 18 }, (_, i) => i);
}
```

## Headless

Headless mode allows you to customize the entire user interface instead of the default elements.

```typescript
import { Component } from '@angular/core';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="showDialog()"><i class="pi pi-user"></i>Login</button>
            <p-dialog maskStyleClass="backdrop-blur-xs" [(visible)]="visible" styleClass="border-0! bg-transparent!">
                <ng-template #headless>
                    <div class="flex flex-col px-8 py-8 gap-5 rounded-2xl" style="border-radius: 12px; background-image: radial-gradient(circle at left top, var(--p-primary-400), var(--p-primary-700))">
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
                        <div class="inline-flex flex-col gap-2">
                            <label for="username" class="text-primary-50 font-semibold text-sm">Username</label>
                            <input pInputText id="username" class="bg-white/20! border-0! p-3! text-primary-50! w-80" />
                        </div>
                        <div class="inline-flex flex-col gap-2">
                            <label for="password" class="text-primary-50 font-semibold text-sm">Password</label>
                            <input pInputText id="password" class="bg-white/20! border-0! p-3! text-primary-50! w-80" type="password" />
                        </div>
                        <div class="flex items-center gap-4">
                            <button type="button" pButton (click)="closeDialog()" [text]="true" class="p-3! w-full text-primary-50! border! border-white/30! hover:bg-white/10!">Cancel</button>
                            <button type="button" pButton (click)="closeDialog()" [text]="true" class="p-3! w-full text-primary-50! border! border-white/30! hover:bg-white/10!">Sign-In</button>
                        </div>
                    </div>
                </ng-template>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule]
})
export class DialogHeadlessDemo {
    visible: boolean = false;

    showDialog() {
        this.visible = true;
    }

    closeDialog() {
        this.visible = false;
    }
}
```

## Long Content

When content exceeds the available space, the content area scrolls while the header and footer stay fixed.

```typescript
import { Component } from '@angular/core';
import { DialogModule } from 'primeng/dialog';
import { TagModule } from 'primeng/tag';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">Inside Scroll</button>
            <p-dialog [(visible)]="visible" [modal]="true" header="Terms of Service" [style]="{ width: '40rem' }">
                <div class="flex flex-col gap-4">
                    <div class="flex items-center gap-2">
                        <p-tag value="Updated" severity="info" />
                        <span class="text-sm text-surface-500 dark:text-surface-400">Last revised: January 1, 2026</span>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">1. Acceptance of Terms</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            By accessing and using this service, you accept and agree to be bound by the terms and provision of this agreement. If you do not agree to abide by the above, please do not use this service.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">2. Use License</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            Permission is granted to temporarily download one copy of the materials on this service for personal, non-commercial transitory viewing only. This is the grant of a license, not a transfer of title, and under this
                            license you may not modify or copy the materials, use the materials for any commercial purpose, or attempt to decompile or reverse engineer any software contained on this service.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">3. Disclaimer</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            The materials on this service are provided on an 'as is' basis. We make no warranties, expressed or implied, and hereby disclaim and negate all other warranties including, without limitation, implied warranties or
                            conditions of merchantability, fitness for a particular purpose, or non-infringement of intellectual property or other violation of rights.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">4. Limitations</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            In no event shall we or our suppliers be liable for any damages (including, without limitation, damages for loss of data or profit, or due to business interruption) arising out of the use or inability to use the
                            materials on this service, even if we or an authorized representative has been notified orally or in writing of the possibility of such damage.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">5. Privacy Policy</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            Your privacy is important to us. Our Privacy Policy explains how we collect, use, and protect your personal information when you use our service. By using our service, you agree to the collection and use of information
                            in accordance with our Privacy Policy. We are committed to ensuring that your information is secure and handled responsibly.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">6. User Accounts</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            When you create an account with us, you must provide information that is accurate, complete, and current at all times. Failure to do so constitutes a breach of the Terms, which may result in immediate termination of
                            your account on our service. You are responsible for safeguarding the password that you use to access the service and for any activities or actions under your password.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">7. Intellectual Property</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            The service and its original content, features, and functionality are and will remain the exclusive property of the company and its licensors. The service is protected by copyright, trademark, and other laws of both
                            domestic and foreign countries. Our trademarks and trade dress may not be used in connection with any product or service without the prior written consent of the company.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">8. Termination</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            We may terminate or suspend your account immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms. Upon termination, your right to use the service
                            will immediately cease. If you wish to terminate your account, you may simply discontinue using the service or contact us to request account deletion.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">9. Indemnification</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            You agree to defend, indemnify, and hold harmless the company and its licensee and licensors, and their employees, contractors, agents, officers, and directors, from and against any and all claims, damages,
                            obligations, losses, liabilities, costs or debt, and expenses, including but not limited to attorney's fees, resulting from or arising out of your use and access of the service, or a breach of these Terms.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">10. Governing Law</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            These terms and conditions are governed by and construed in accordance with applicable laws and you irrevocably submit to the exclusive jurisdiction of the courts in that location. Any claim relating to this service
                            shall be governed by the laws without regard to its conflict of law provisions.
                        </p>
                    </div>
                    <div>
                        <h3 class="text-base font-semibold mt-0 mb-2">11. Changes to Terms</h3>
                        <p class="text-sm leading-relaxed mt-0 mb-0 text-surface-600 dark:text-surface-300">
                            We reserve the right, at our sole discretion, to modify or replace these Terms at any time. If a revision is material we will try to provide at least 30 days' notice prior to any new terms taking effect. What
                            constitutes a material change will be determined at our sole discretion. By continuing to access or use our service after those revisions become effective, you agree to be bound by the revised terms.
                        </p>
                    </div>
                </div>
                <ng-template #footer>
                    <button type="button" pButton severity="secondary" (click)="visible = false">Decline</button>
                    <button type="button" pButton (click)="visible = false">Accept</button>
                </ng-template>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, TagModule, ButtonModule]
})
export class DialogLongContentDemo {
    visible: boolean = false;
}
```

## Maximizable

Setting maximizable property to true enables the full screen mode.

```typescript
import { Component } from '@angular/core';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="showDialog()">Show</button>
            <p-dialog header="Header" [modal]="true" [(visible)]="visible" [style]="{ width: '50rem' }" [breakpoints]="{ '1199px': '75vw', '575px': '90vw' }" [maximizable]="true">
                <p class="text-sm">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                    consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                </p>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule]
})
export class DialogMaximizableDemo {
    visible: boolean = false;

    showDialog() {
        this.visible = true;
    }
}
```

## Modal

Mask layer behind the Dialog is enabled with the modal property, and dismissableMask allows closing it by clicking outside.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Dialog, DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">Edit Profile</button>
            <p-dialog [(visible)]="visible" [modal]="true" [dismissableMask]="true" header="Edit Profile" [style]="{ width: '25rem' }">
                <div class="flex flex-col gap-6">
                    <span class="text-surface-500 dark:text-surface-400">Update your information. Click outside the dialog to dismiss.</span>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="username" class="font-semibold">Username</label>
                        <input pInputText id="username" [(ngModel)]="username" />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="email" class="font-semibold">Email</label>
                        <input pInputText id="email" [(ngModel)]="email" />
                    </div>
                    <div class="flex justify-end gap-2">
                        <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                        <button type="button" pButton (click)="visible = false">Save</button>
                    </div>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule, LabelModule, FormsModule]
})
export class DialogModalDemo {
    visible: boolean = false;

    username: string = '';

    email: string = '';
}
```

## overlaysinside-doc

When dialog includes other components with overlays such as dropdown, the overlay part cannot exceed dialog boundaries due to overflow. In order to solve this, you can either append the overlay to the body by using appendTo property or allow overflow in dialog.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { SelectModule } from 'primeng/select';
import { ButtonModule } from 'primeng/button';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="showDialog()"><i class="pi pi-external-link"></i>Show</button>
            <p-dialog header="Header" [(visible)]="visible" [style]="{ width: '50vw' }">
                <div class="flex py-2 justify-center">
                    <p-select appendTo="body" [options]="cities" [(ngModel)]="selectedCity" placeholder="Select a City" optionLabel="name"></p-select>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, SelectModule, ButtonModule, FormsModule]
})
export class DialogOverlaysInsideDemo implements OnInit {
    cities: City[] | undefined;

    selectedCity: City | undefined;

    visible: boolean = false;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }

    showDialog() {
        this.visible = true;
    }
}
```

## Position

The position property is used to display a Dialog at all edges and corners of the screen.

```typescript
import { Component } from '@angular/core';
import { Dialog, DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { DialogPosition } from 'primeng/types/dialog';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-2 mb-2">
            <button type="button" pButton (click)="showDialog('left')" severity="secondary" class="min-w-40">Left</button>
            <button type="button" pButton (click)="showDialog('right')" severity="secondary" class="min-w-40">Right</button>
        </div>
        <div class="flex flex-wrap justify-center gap-2 mb-2">
            <button type="button" pButton (click)="showDialog('topleft')" severity="secondary" class="min-w-40">TopLeft</button>
            <button type="button" pButton (click)="showDialog('top')" severity="secondary" class="min-w-40">Top</button>
            <button type="button" pButton (click)="showDialog('topright')" severity="secondary" class="min-w-40">TopRight</button>
        </div>
        <div class="flex flex-wrap justify-center gap-2">
            <button type="button" pButton (click)="showDialog('bottomleft')" severity="secondary" class="min-w-40">BottomLeft</button>
            <button type="button" pButton (click)="showDialog('bottom')" severity="secondary" class="min-w-40">Bottom</button>
            <button type="button" pButton (click)="showDialog('bottomright')" severity="secondary" class="min-w-40">BottomRight</button>
        </div>
        <p-dialog header="Edit Profile" [modal]="true" [(visible)]="visible" [position]="position" [style]="{ width: '25rem' }">
            <span class="text-surface-500 dark:text-surface-400 block mb-8 text-sm">Update your information.</span>
            <div class="flex items-center gap-4 mb-4">
                <label for="username" class="font-semibold w-24 text-sm">Username</label>
                <input pInputText id="username" class="flex-auto" autocomplete="off" />
            </div>
            <div class="flex items-center gap-4 mb-8">
                <label for="email" class="font-semibold w-24 text-sm">Email</label>
                <input pInputText id="email" class="flex-auto" autocomplete="off" />
            </div>
            <div class="flex justify-end gap-2">
                <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                <button type="button" pButton (click)="visible = false">Save</button>
            </div>
        </p-dialog>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule]
})
export class DialogPositionDemo {
    visible: boolean = false;

    position: DialogPosition = 'center';

    showDialog(position: DialogPosition) {
        this.position = position;
        this.visible = true;
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">Edit Profile</button>
            <p-dialog [(visible)]="visible" [modal]="true" header="Edit Profile" [style]="{ width: '28rem' }">
                <div class="flex flex-col gap-6">
                    <div class="flex items-center gap-4">
                        <p-avatar image="https://primefaces.org/cdn/primevue/images/avatar/asiyajavayant.png" shape="circle" size="xlarge" />
                        <div>
                            <p class="font-semibold! text-lg! mt-0! mb-1!">Amanda Miller</p>
                            <p class="text-surface-500 dark:text-surface-400 mt-0! mb-0! text-sm!">amanda&#64;example.com</p>
                        </div>
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="name" class="text-sm font-semibold">Name</label>
                        <input pInputText id="name" [(ngModel)]="name" />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="username" class="text-sm font-semibold">Username</label>
                        <input pInputText id="username" [(ngModel)]="username" />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="email" class="text-sm font-semibold">Email</label>
                        <input pInputText id="email" [(ngModel)]="email" />
                    </div>
                    <div class="flex justify-end gap-2">
                        <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                        <button type="button" pButton (click)="visible = false">Save Changes</button>
                    </div>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, DialogModule, ButtonModule, InputTextModule, LabelModule, FormsModule]
})
export class DialogPreviewDemo {
    visible: boolean = false;

    name: string = 'Amanda Miller';

    username: string = '@amandamiller';

    email: string = 'amanda@example.com';
}
```

## Responsive

Dialog dimensions can be made responsive with breakpoint-based utility classes applied directly to the component.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="visible = true">New Event</button>
            <p-dialog [(visible)]="visible" [modal]="true" header="Create Event" styleClass="w-[90vw] md:w-[75vw] lg:w-[50vw]">
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-1">
                        <label pLabel for="eventName" class="text-sm font-semibold">Event Name</label>
                        <input pInputText id="eventName" [(ngModel)]="eventName" placeholder="e.g. Team Standup" />
                    </div>
                    <div class="flex flex-col md:flex-row gap-4">
                        <div class="flex flex-col gap-1 flex-1">
                            <label pLabel for="organizer" class="text-sm font-semibold">Organizer</label>
                            <input pInputText id="organizer" [(ngModel)]="organizer" placeholder="Name" />
                        </div>
                        <div class="flex flex-col gap-1 flex-1">
                            <label pLabel for="email" class="text-sm font-semibold">Email</label>
                            <input pInputText id="email" [(ngModel)]="email" placeholder="organizer@example.com" />
                        </div>
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="location" class="text-sm font-semibold">Location</label>
                        <input pInputText id="location" [(ngModel)]="location" placeholder="Add a location or video link" />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="description" class="text-sm font-semibold">Description</label>
                        <input pInputText id="description" [(ngModel)]="description" placeholder="Event details" />
                    </div>
                    <div class="flex flex-col sm:flex-row justify-end gap-2 mt-2">
                        <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                        <button type="button" pButton (click)="visible = false">Create Event</button>
                    </div>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule, LabelModule, FormsModule]
})
export class DialogResponsiveDemo {
    visible: boolean = false;

    eventName: string = '';

    organizer: string = '';

    email: string = '';

    location: string = '';

    description: string = '';
}
```

## Template

Dialog can be customized using header and footer templates.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="showDialog()">Show</button>
            <p-dialog [(visible)]="visible" [modal]="true" [style]="{ width: '25rem' }">
                <ng-template #header>
                    <div class="inline-flex items-center justify-center gap-2">
                        <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape="circle" />
                        <span class="font-bold whitespace-nowrap">Amy Elsner</span>
                    </div>
                </ng-template>
                <span class="text-surface-500 dark:text-surface-400 block mb-8 text-sm">Update your information.</span>
                <div class="flex items-center gap-4 mb-4">
                    <label for="username" class="font-semibold w-24 text-sm">Username</label>
                    <input pInputText id="username" class="flex-auto" autocomplete="off" />
                </div>
                <div class="flex items-center gap-4 mb-2">
                    <label for="email" class="font-semibold w-24 text-sm">Email</label>
                    <input pInputText id="email" class="flex-auto" autocomplete="off" />
                </div>
                <ng-template #footer>
                    <button type="button" pButton [text]="true" severity="secondary" (click)="visible = false">Cancel</button>
                    <button type="button" pButton [outlined]="true" severity="secondary" (click)="visible = false">Save</button>
                </ng-template>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, DialogModule, ButtonModule, InputTextModule]
})
export class DialogTemplateDemo {
    visible: boolean = false;

    showDialog() {
        this.visible = true;
    }
}
```

## Without Modal

Mask layer behind the Dialog is configured with the modal property. By default, no modal layer is added.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Dialog, DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <button type="button" pButton (click)="showDialog()">Show</button>
            <p-dialog header="Edit Profile" [(visible)]="visible" [style]="{ width: '25rem' }">
                <div class="flex flex-col gap-6">
                    <span class="text-surface-500 dark:text-surface-400">Update your information.</span>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="username" class="font-semibold">Username</label>
                        <input pInputText id="username" [(ngModel)]="username" />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label pLabel for="email" class="font-semibold">Email</label>
                        <input pInputText id="email" [(ngModel)]="email" />
                    </div>
                    <div class="flex justify-end gap-2">
                        <button type="button" pButton severity="secondary" (click)="visible = false">Cancel</button>
                        <button type="button" pButton (click)="visible = false">Save</button>
                    </div>
                </div>
            </p-dialog>
        </div>
    `,
    standalone: true,
    imports: [DialogModule, ButtonModule, InputTextModule, LabelModule, FormsModule]
})
export class DialogWithoutModalDemo {
    visible: boolean = false;

    username: string = '';

    email: string = '';

    showDialog() {
        this.visible = true;
    }
}
```

## Dialog

Dialog is a container to display content in an overlay window.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, DialogPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| header | string \| undefined | - | Title text of the dialog. |
| draggable | boolean | - | Enables dragging to change the position using header. |
| resizable | boolean | - | Enables resizing of the content. |
| contentStyle | { [klass: string]: any } \| null \| undefined | - | Style of the content section. |
| contentStyleClass | string \| undefined | - | Style class of the content. |
| modal | boolean | - | Defines if background should be blocked when dialog is displayed. |
| closeOnEscape | boolean | - | Specifies if pressing escape key should hide the dialog. |
| dismissableMask | boolean | - | Specifies if clicking the modal background should hide the dialog. |
| rtl | boolean | - | When enabled dialog is displayed in RTL direction. |
| closable | boolean | - | Adds a close icon to the header to hide the dialog. |
| breakpoints | Record<string, string> \| undefined | - | Object literal to define widths per screen size. |
| styleClass | string \| undefined | - | Style class of the component. |
| maskStyleClass | string \| undefined | - | Style class of the mask. |
| maskStyle | { [klass: string]: any } \| null \| undefined | - | Style of the mask. |
| showHeader | boolean | - | Whether to show the header or not. |
| blockScroll | boolean | - | Whether background scroll should be blocked when dialog is visible. |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| minX | number | - | Minimum value for the left coordinate of dialog in dragging. |
| minY | number | - | Minimum value for the top coordinate of dialog in dragging. |
| focusOnShow | boolean | - | When enabled, first focusable element receives focus on show. |
| maximizable | boolean | - | Whether the dialog can be displayed full screen. |
| keepInViewport | boolean | - | Keeps dialog in the viewport. |
| focusTrap | boolean | - | When enabled, can only focus on elements inside the dialog. |
| maskMotionOptions | MotionOptions \| undefined | - | The motion options for the mask. |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| closeIcon | string \| undefined | - | Name of the close icon. |
| closeAriaLabel | string \| undefined | - | Defines a string that labels the close button for accessibility. |
| closeTabindex | string | - | Index of the close button in tabbing order. |
| minimizeIcon | string \| undefined | - | Name of the minimize icon. |
| maximizeIcon | string \| undefined | - | Name of the maximize icon. |
| closeButtonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the underlying button. |
| maximizeButtonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the underlying button. |
| visible | boolean | - | Specifies the visibility of the dialog. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| position | DialogPosition \| undefined | - | Position of the dialog, options are "center", "top", "bottom", "left", "right", "topleft", "topright", "bottomleft" or "bottomright". |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'self' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| role | value: string | Role attribute of html element. |
| onShow | value: any | Callback to invoke when dialog is shown. |
| onHide | value: any | Callback to invoke when dialog is hidden. |
| onResizeInit | event: MouseEvent | Callback to invoke when dialog resizing is initiated. |
| onResizeEnd | event: MouseEvent | Callback to invoke when dialog resizing is completed. |
| onDragStart | event: MouseEvent | Callback to invoke when dialog dragging is initiated. |
| onDragEnd | event: DragEvent | Callback to invoke when dialog dragging is completed. |
| onMaximize | value: { maximized: boolean \| undefined } | Callback to invoke when dialog maximized or unmaximized. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<DialogHeaderTemplateContext> \| undefined | Custom header template. |
| content | TemplateRef<void> \| undefined | Custom content template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| closeicon | TemplateRef<void> \| undefined | Custom close icon template. |
| maximizeicon | TemplateRef<void> \| undefined | Custom maximize icon template. |
| minimizeicon | TemplateRef<void> \| undefined | Custom minimize icon template. |
| headless | TemplateRef<void> \| undefined | Custom headless template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| mask | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the mask's DOM element. |
| resizeHandle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the resize handle's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| title | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the title's DOM element. |
| headerActions | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header actions' DOM element. |
| pcMaximizeButton | ButtonPassThrough | Used to pass attributes to the maximize Button component. |
| pcCloseButton | ButtonPassThrough | Used to pass attributes to the close Button component. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |
| motion | MotionOptions | Used to pass motion options for the dialog animation. |
| maskMotion | MotionOptions | Used to pass motion options for the mask animation. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-dialog-mask | Class name of the mask element |
| p-dialog | Class name of the root element |
| p-dialog-header | Class name of the header element |
| p-dialog-title | Class name of the title element |
| p-dialog-header-actions | Class name of the header actions element |
| p-dialog-maximize-button | Class name of the maximize button element |
| p-dialog-close-button | Class name of the close button element |
| p-dialog-content | Class name of the content element |
| p-dialog-footer | Class name of the footer element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| dialog.background | --p-dialog-background | Background of root |
| dialog.border.color | --p-dialog-border-color | Border color of root |
| dialog.color | --p-dialog-color | Color of root |
| dialog.border.radius | --p-dialog-border-radius | Border radius of root |
| dialog.shadow | --p-dialog-shadow | Shadow of root |
| dialog.header.padding | --p-dialog-header-padding | Padding of header |
| dialog.header.gap | --p-dialog-header-gap | Gap of header |
| dialog.title.font.size | --p-dialog-title-font-size | Font size of title |
| dialog.title.font.weight | --p-dialog-title-font-weight | Font weight of title |
| dialog.content.padding | --p-dialog-content-padding | Padding of content |
| dialog.footer.padding | --p-dialog-footer-padding | Padding of footer |
| dialog.footer.gap | --p-dialog-footer-gap | Gap of footer |
