---
title: "Angular Animate On Scroll Directive"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/animateonscroll.md"
description: "AnimateOnScroll is used to apply animations to elements when entering or leaving the viewport during scrolling."
---

# Angular Animate On Scroll Directive

AnimateOnScroll is used to apply animations to elements when entering or leaving the viewport during scrolling.

## Accessibility

Screen Reader AnimateOnScroll does not require any roles and attributes. Keyboard Support Component does not include any interactive elements.

## basic-doc

Animation classes are defined with the enterClass and leaveClass properties. This example utilizes tailwindcss-primeui plugin animations however any valid CSS animation is supported.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { AnimateOnScrollModule } from 'primeng/animateonscroll';
import { ArrowDown } from '@primeicons/angular/arrow-down';
import { User } from '@primeicons/angular/user';
import { Users } from '@primeicons/angular/users';
import { Building } from '@primeicons/angular/building';
import { Wifi } from '@primeicons/angular/wifi';
import { Database } from '@primeicons/angular/database';
import { ArrowsV } from '@primeicons/angular/arrows-v';

@Component({
    template: `
        <div class="flex flex-col items-center overflow-hidden">
            <div class="flex flex-col items-center gap-4">
                <div class="text-2xl font-medium">Scroll Down</div>
                <div class="animate-bounce h-8 w-8 bg-primary text-primary-contrast rounded-full inline-flex items-center justify-center">
                    <svg data-p-icon="arrow-down"></svg>
                </div>
            </div>
            <div class="h-120"></div>
            <div class="flex flex-wrap justify-center gap-8">
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 slide-in-from-l-8 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col border border-surface shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <div class="rounded-full bg-primary text-primary-contrast w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="user" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Individual</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col border border-surface shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <div class="rounded-full bg-primary text-primary-contrast w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="users" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Team</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div pAnimateOnScroll enterClass="animate-enter fade-in-10 slide-in-from-r-8 animate-duration-1000" class="flex flex-col border border-surface shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4">
                    <div class="rounded-full bg-primary text-primary-contrast w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="building" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Enterprise</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
            </div>
            <div class="h-120"></div>
            <div class="flex flex-wrap justify-center gap-8">
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 slide-in-from-t-20 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col border border-primary-200 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape="circle" size="xlarge" />
                    <span class="text-2xl font-medium">Jenna Thompson</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 slide-in-from-b-20 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col border border-primary-200 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/asiyajavayant.png" shape="circle" size="xlarge" />
                    <span class="text-2xl font-medium">Isabel Garcia</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 slide-in-from-t-20 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col border border-primary-200 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <p-avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/onyamalimba.png" shape="circle" size="xlarge" />
                    <span class="text-2xl font-medium">Xavier Mason</span>
                    <span class="text-muted-color text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
            </div>
            <div class="h-120"></div>
            <div class="flex flex-wrap justify-center gap-8">
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 spin-in-45 slide-in-from-t-12 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col bg-primary text-primary-contrast border-primary shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <span class="bg-white/20 text-xl font-medium rounded-xl px-4 py-2">850K</span>
                    <span class="text-2xl font-bold">Customers</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 zoom-in-50 slide-in-from-t-20 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col bg-primary text-primary-contrast border-primary shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <span class="bg-white/20 text-xl font-medium rounded-xl px-4 py-2">$1.5M</span>
                    <span class="text-2xl font-bold">Revenue</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 spin-in-[-45deg] slide-in-from-t-16 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col bg-primary text-primary-contrast border-primary shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <span class="bg-white/20 text-xl font-medium rounded-xl px-4 py-2">140K</span>
                    <span class="text-2xl font-bold">Sales</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
            </div>
            <div class="h-120"></div>
            <div class="flex flex-wrap justify-center gap-8">
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 zoom-in-50 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col bg-purple-500 text-white border-purple-500 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <div class="rounded-full border-2 border-white w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="wifi" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Bandwidth</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 zoom-in-75 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-1000"
                    class="flex flex-col bg-teal-500 text-white border-teal-500 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <div class="rounded-full border-2 border-white w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="database" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Storage</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
                <div
                    pAnimateOnScroll
                    enterClass="animate-enter fade-in-10 zoom-in-50 animate-duration-1000"
                    leaveClass="animate-leave fade-out-0 animate-duration-100"
                    class="flex flex-col bg-indigo-500 text-white border-indigo-500 shadow-lg justify-center items-center max-w-80 rounded-2xl p-8 gap-4"
                >
                    <div class="rounded-full border-2 border-white w-12 h-12 flex items-center justify-center">
                        <svg data-p-icon="arrows-v" class="text-2xl!"></svg>
                    </div>
                    <span class="text-2xl font-bold">Requests</span>
                    <span class="text-center">Lorem ipsum dolor sit amet consectetur adipisicing elit.</span>
                </div>
            </div>
            <div class="h-60"></div>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, AnimateOnScrollModule, ArrowDown, User, Users, Building, Wifi, Database, ArrowsV]
})
export class AnimateOnScrollBasicDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { AnimateOnScrollModule } from 'primeng/animateonscroll';
import { ArrowDown } from '@primeicons/angular/arrow-down';
import { ArrowRight } from '@primeicons/angular/arrow-right';

@Component({
    template: `
        <div class="flex flex-col items-center overflow-hidden">
            <div class="flex flex-col items-center gap-4">
                <div class="text-2xl font-medium">Scroll Down</div>
                <div class="animate-bounce h-8 w-8 bg-primary text-primary-contrast rounded-full inline-flex items-center justify-center">
                    <svg data-p-icon="arrow-down"></svg>
                </div>
            </div>
            <div class="h-180"></div>
            <div class="flex flex-col items-center gap-8 w-full">
                <div pAnimateOnScroll enterClass="animate-enter fade-in-10 slide-in-from-b-16 animate-duration-1000" leaveClass="animate-leave fade-out-0 animate-duration-100">
                    <div class="text-5xl lg:text-[4rem] text-center font-bold max-w-lg lg:max-w-3xl text-surface-900 dark:text-surface-50">Discover real-world design inspiration.</div>
                </div>
                <div pAnimateOnScroll enterClass="animate-enter fade-in-10 slide-in-from-b-16 animate-duration-1000" leaveClass="animate-leave fade-out-0 animate-duration-100">
                    <div class="max-w-md lg:max-w-md text-base lg:text-lg text-center text-muted-color">Featuring over 400.000 screens and 1,000 iOS, Android & Web apps — New content weekly.</div>
                </div>
                <div pAnimateOnScroll enterClass="animate-enter fade-in-10 slide-in-from-b-16 animate-duration-1000" leaveClass="animate-leave fade-out-0 animate-duration-100">
                    <div class="flex items-center justify-center gap-4 font-semibold">
                        <button pButton size="large" rounded>Join for free</button>
                        <button pButton size="large" variant="outlined" rounded severity="primary">
                            See our plans
                            <span class="bg-surface-200 dark:bg-surface-800 size-6 inline-flex items-center justify-center rounded-full">
                                <svg data-p-icon="arrow-right" class="font-bold! text-sm!"></svg>
                            </span>
                        </button>
                    </div>
                </div>
            </div>
            <div class="h-60"></div>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule, AnimateOnScrollModule, ArrowDown, ArrowRight]
})
export class AnimateOnScrollPreviewDemo {}
```

## Animate On Scroll

AnimateOnScroll is used to apply animations to elements when entering or leaving the viewport during scrolling.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| enterClass | string \| undefined | - | Selector to define the CSS class for enter animation. |
| leaveClass | string \| undefined | - | Selector to define the CSS class for leave animation. |
| root | HTMLElement \| null \| undefined | - | Specifies the root option of the IntersectionObserver API. |
| rootMargin | string \| undefined | - | Specifies the rootMargin option of the IntersectionObserver API. |
| threshold | number | - | Specifies the threshold option of the IntersectionObserver API |
| once | boolean | - | Whether the scroll event listener should be removed after initial run. |
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | any | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
