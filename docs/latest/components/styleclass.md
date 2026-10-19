---
title: "Angular StyleClass Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/styleclass.md"
description: "StyleClass manages css classes declaratively to during enter/leave animations or just to toggle classes on an element."
---

# Angular StyleClass Component

StyleClass manages css classes declaratively to during enter/leave animations or just to toggle classes on an element.

## Animation

Classes to apply during enter and leave animations are specified using the enterFromClass , enterActiveClass , enterToClass , leaveFromClass , leaveActiveClass , leaveToClass properties. In addition in case the target is an overlay, hideOnOutsideClick would be handy to hide the target if outside of the popup is clicked, or enable hideOnEscape to close the popup by listening escape key.

```typescript
import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { StyleClassModule } from 'primeng/styleclass';

@Component({
    template: `
        <div class="flex items-center justify-center gap-7">
            <div class="flex flex-col items-center">
                <div>
                    <button type="button" pButton pStyleClass=".box1" enterFromClass="my-hidden" enterActiveClass="my-fadein" class="mr-2">FadeIn</button>
                    <button type="button" pButton pStyleClass=".box1" leaveActiveClass="my-fadeout" leaveToClass="my-hidden" severity="secondary">FadeOut</button>
                </div>
                <div class="h-32">
                    <div class="my-hidden animate-duration-500 box1">
                        <div class="flex bg-primary text-primary-contrast items-center justify-center py-3 rounded-md mt-4 font-bold text-sm w-28 h-28">Custom</div>
                    </div>
                </div>
            </div>
            <div class="flex flex-col items-center">
                <div>
                    <button type="button" pButton pStyleClass=".box2" enterFromClass="hidden" enterActiveClass="animate-slidedown" class="mr-2">SlideDown</button>
                    <button type="button" pButton pStyleClass=".box2" leaveActiveClass="animate-slideup" leaveToClass="hidden" severity="secondary">SlideUp</button>
                </div>
                <div class="h-32">
                    <div class="hidden animate-duration-500 box2 overflow-hidden">
                        <div class="flex bg-primary text-primary-contrast items-center justify-center py-3 rounded-md mt-4 font-bold text-sm w-28 h-28">Content</div>
                    </div>
                </div>
            </div>
        </div>
    `,
    styles: [
        `
            :host ::ng-deep {
                @keyframes my-fadein {
                    0% {
                        opacity: 0;
                    }
                    100% {
                        opacity: 1;
                    }
                }

                @keyframes my-fadeout {
                    0% {
                        opacity: 1;
                    }
                    100% {
                        opacity: 0;
                    }
                }

                .my-hidden {
                    display: none;
                }

                .my-fadein {
                    animation: my-fadein 150ms linear;
                }

                .my-fadeout {
                    animation: my-fadeout 150ms linear;
                }
            }
        `
    ],
    standalone: true,
    imports: [ButtonModule, StyleClassModule]
})
export class StyleClassAnimationDemo {}
```

## Basic

StyleClass applies enter and leave animations to a target element declaratively. The target is resolved with the selector option, here &#64;next referring to the next sibling.

```typescript
import { Component } from '@angular/core';
import { StyleClassModule } from 'primeng/styleclass';
import { ChevronDown } from '@primeicons/angular/chevron-down';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4 h-40">
            <button
                type="button"
                pStyleClass="@next"
                enterFromClass="hidden"
                enterActiveClass="animate-scalein"
                leaveToClass="hidden"
                leaveActiveClass="animate-fadeout"
                [hideOnOutsideClick]="true"
                class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-contrast hover:bg-primary/90 transition-colors cursor-pointer text-sm font-medium"
            >
                Toggle Panel
                <svg data-p-icon="chevron-down" [size]="14" />
            </button>
            <div class="hidden w-full max-w-md p-4 rounded-lg border border-surface-200 dark:border-surface-700 bg-surface-0 dark:bg-surface-900 shadow-md origin-top">
                <p class="text-sm text-surface-700 dark:text-surface-200 leading-relaxed m-0">This panel is toggled using StyleClass with scale-in and fade-out animations. Click the button again or click outside to dismiss.</p>
            </div>
        </div>
    `,
    standalone: true,
    imports: [StyleClassModule, ChevronDown]
})
export class StyleClassBasicDemo {}
```

## Hide On Resize

When hideOnResize is enabled, the leave animation is triggered automatically when resizing occurs. Use the resizeSelector property to specify whether to listen to window resize events or element-specific resize events. Set resizeSelector to "window" (default) or "document" for browser resize, or a CSS selector to observe the target element's dimensions.

```typescript
import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { StyleClassModule } from 'primeng/styleclass';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <div class="flex flex-col items-center gap-4 w-100">
                <button type="button" pButton pStyleClass=".window-responsive-box" enterFromClass="hidden" enterActiveClass="animate-fadein" leaveActiveClass="animate-fadeout" leaveToClass="hidden" [hideOnResize]="true" resizeSelector="window">
                    Show Window Responsive Content
                </button>
                <div class="window-responsive-box hidden animate-duration-300 border border-lg border-surface">
                    <div class="p-3 flex flex-col gap-2">
                        <h3 class="text-lg font-bold">Window Responsive Panel</h3>
                        <p class="text-sm">This panel will hide when you resize the browser window.</p>
                        <p class="text-sm">Try resizing your browser window to see the effect.</p>
                    </div>
                </div>
            </div>
            <div class="flex flex-col items-center gap-4 w-100">
                <button
                    type="button"
                    pButton
                    pStyleClass=".resizable-container"
                    enterFromClass="hidden"
                    enterActiveClass="animate-fadein"
                    leaveActiveClass="animate-fadeout"
                    leaveToClass="hidden"
                    [hideOnResize]="true"
                    resizeSelector=".resizable-container"
                >
                    Show Resizable Panel
                </button>
                <div class="resizable-container hidden animate-duration-300 border border-lg border-surface w-80 w-max-[25rem] w-min-[15rem] overflow-auto resize">
                    <div class="p-4 h-full flex flex-col gap-2">
                        <h3 class="text-lg font-bold">Resizable Panel</h3>
                        <p class="text-sm">Drag the resize handle in the bottom-right corner to resize this panel.</p>
                        <p class="text-sm">The panel will hide when you resize it.</p>
                    </div>
                </div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule, StyleClassModule]
})
export class StyleClassHideOnResizeDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { StyleClassModule } from 'primeng/styleclass';
import { Sparkles } from '@primeicons/angular/sparkles';

@Component({
    template: `
        <div class="flex flex-col items-center gap-3 h-60">
            <button
                type="button"
                pStyleClass="@next"
                enterFromClass="hidden"
                enterActiveClass="p-anchored-overlay-enter-active"
                leaveToClass="hidden"
                leaveActiveClass="p-anchored-overlay-leave-active"
                [hideOnOutsideClick]="true"
                class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-surface-100 dark:bg-surface-800 hover:bg-surface-200 dark:hover:bg-surface-700 transition-colors cursor-pointer text-sm font-medium"
            >
                <svg data-p-icon="sparkles" [size]="14" class="text-primary" />
                What's new
                <span class="text-xs text-muted-color tabular-nums">v2.4</span>
            </button>
            <div class="hidden w-72 rounded-lg border border-surface bg-surface-0 dark:bg-surface-900 shadow-md origin-top">
                <div class="px-4 py-3 border-b border-surface">
                    <div class="font-semibold text-sm">Release 2.4</div>
                    <div class="text-xs text-muted-color">May 6, 2026</div>
                </div>
                <ul class="p-3 space-y-2 list-none m-0">
                    @for (c of changes; track c.label) {
                        <li class="flex items-start gap-2 text-sm">
                            <span [class]="'shrink-0 text-[10px] uppercase tracking-wide font-semibold px-1.5 py-0.5 rounded ' + tagColor[c.type]">{{ c.type }}</span>
                            <span class="text-color">{{ c.label }}</span>
                        </li>
                    }
                </ul>
            </div>
        </div>
    `,
    standalone: true,
    imports: [StyleClassModule, Sparkles]
})
export class StyleClassPreviewDemo {
    changes = [
        { type: 'New', label: 'AI-powered command palette' },
        { type: 'Improved', label: 'Faster theme switching' },
        { type: 'Fixed', label: 'Crash on empty filter' }
    ];

    tagColor: Record<string, string> = {
        New: 'bg-emerald-500/10 text-emerald-500',
        Improved: 'bg-sky-500/10 text-sky-500',
        Fixed: 'bg-amber-500/10 text-amber-500'
    };
}
```

## Selector

The target element is resolved with the selector option, which accepts keywords such as &#64;next , &#64;prev , &#64;parent and &#64;grandparent , or any valid CSS selector.

```typescript
import { Component } from '@angular/core';
import { StyleClassModule } from 'primeng/styleclass';

@Component({
    template: `
        <div class="flex flex-col gap-8">
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium text-surface-500 dark:text-surface-400">&#64;next</span>
                <div class="flex items-center gap-4">
                    <button
                        type="button"
                        pStyleClass="@next"
                        enterFromClass="hidden"
                        enterActiveClass="animate-fadein"
                        leaveActiveClass="animate-fadeout"
                        leaveToClass="hidden"
                        class="px-4 py-2 rounded-lg bg-primary text-primary-contrast hover:bg-primary/90 transition-colors cursor-pointer text-sm font-medium shrink-0"
                    >
                        &#64;next
                    </button>
                    <div class="hidden px-4 py-2 rounded-lg bg-primary/10 text-primary text-sm font-medium">Next Sibling</div>
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium text-surface-500 dark:text-surface-400">&#64;prev</span>
                <div class="flex items-center gap-4">
                    <div class="hidden px-4 py-2 rounded-lg bg-primary/10 text-primary text-sm font-medium">Previous Sibling</div>
                    <button
                        type="button"
                        pStyleClass="@prev"
                        enterFromClass="hidden"
                        enterActiveClass="animate-fadein"
                        leaveActiveClass="animate-fadeout"
                        leaveToClass="hidden"
                        class="px-4 py-2 rounded-lg bg-primary text-primary-contrast hover:bg-primary/90 transition-colors cursor-pointer text-sm font-medium shrink-0"
                    >
                        &#64;prev
                    </button>
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium text-surface-500 dark:text-surface-400">CSS Selector</span>
                <div class="flex items-center gap-4">
                    <button
                        type="button"
                        pStyleClass="#remote-target"
                        enterFromClass="hidden"
                        enterActiveClass="animate-scalein"
                        leaveActiveClass="animate-fadeout"
                        leaveToClass="hidden"
                        class="px-4 py-2 rounded-lg bg-primary text-primary-contrast hover:bg-primary/90 transition-colors cursor-pointer text-sm font-medium shrink-0"
                    >
                        #remote-target
                    </button>
                </div>
                <div id="remote-target" class="hidden px-4 py-2 rounded-lg bg-primary/10 text-primary text-sm font-medium origin-top w-fit">Remote Target</div>
            </div>
        </div>
    `,
    standalone: true,
    imports: [StyleClassModule]
})
export class StyleClassSelectorDemo {}
```

## Toggle Class

StyleClass has two modes, toggleClass to simply add-remove a class and enter/leave animations. The target element to change the styling is defined with the selector property that accepts any valid CSS selector or keywords including &#64;next , prev , parent , grandparent

```typescript
import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { StyleClassModule } from 'primeng/styleclass';

@Component({
    template: `
        <div class="flex flex-col items-center">
            <button type="button" pButton pStyleClass="@next" toggleClass="hidden">Toggle Display</button>
            <input type="text" pInputText class="hidden mt-4" />
        </div>
    `,
    standalone: true,
    imports: [ButtonModule, InputTextModule, StyleClassModule]
})
export class StyleClassToggleClassDemo {}
```
