---
title: "Angular Timeline Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/timeline.md"
description: "Timeline visualizes a series of chained events."
---

# Angular Timeline Component

Timeline visualizes a series of chained events.

## Accessibility

Screen Reader Timeline uses a semantic ordered list element to list the events. No specific role is enforced, still you may use any aria role and attributes as any valid attribute is passed to the list element. Keyboard Support Component does not include any interactive elements.

## Activity Feed

Display real-time activity streams with user avatars and contextual information.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { TimelineModule } from 'primeng/timeline';
import { History } from '@primeicons/angular/history';
import { Minus } from '@primeicons/angular/minus';

@Component({
    template: `
        <div class="max-w-2xl">
            <div class="flex items-center gap-3 mb-6">
                <svg data-p-icon="history" [size]="16" class="text-surface-500" />
                <span class="text-lg font-semibold text-surface-900 dark:text-surface-0">Recent Activity</span>
            </div>
            <p-timeline [value]="activities">
                <ng-template #opposite let-activity>
                    <span class="text-xs text-surface-400 dark:text-surface-500 whitespace-nowrap">{{ activity.time }}</span>
                </ng-template>
                <ng-template #marker let-activity>
                    <p-avatar [label]="activity.user.avatar" shape="circle" [class]="activity.user.color + ' text-white! text-xs! font-semibold!'" />
                </ng-template>
                <ng-template #content let-activity>
                    <div class="pb-6">
                        <div class="flex items-center gap-2 flex-wrap">
                            <span class="font-medium text-surface-900 dark:text-surface-0">{{ activity.user.name }}</span>
                            <span class="text-surface-500 dark:text-surface-400">{{ activity.action }}</span>
                            <span class="font-medium text-primary">{{ activity.target }}</span>
                            @if (activity.repo) {
                                <span class="text-surface-500 dark:text-surface-400">to</span>
                                <code class="px-2 py-0.5 rounded bg-surface-100 dark:bg-surface-800 text-sm font-mono text-surface-700 dark:text-surface-300">{{ activity.repo }}</code>
                            }
                        </div>
                        @if (activity.description) {
                            <p class="mt-2! text-sm! text-surface-600 dark:text-surface-400">{{ activity.description }}</p>
                        }
                        @if (activity.details) {
                            <div class="mt-3 p-3 rounded-lg bg-surface-50 dark:bg-surface-800 border border-surface-200 dark:border-surface-700">
                                <ul class="space-y-1">
                                    @for (detail of activity.details; track detail) {
                                        <li class="text-sm text-surface-600 dark:text-surface-400 font-mono flex items-start gap-2">
                                            <svg data-p-icon="minus" class="text-xs mt-1.5 text-surface-400" />
                                            {{ detail }}
                                        </li>
                                    }
                                </ul>
                            </div>
                        }
                    </div>
                </ng-template>
            </p-timeline>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, TimelineModule, History, Minus],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TimelineActivityDemo {
    activities = [
        {
            id: 1,
            user: { name: 'Sarah Chen', avatar: 'SC', color: 'bg-violet-500!' },
            action: 'pushed',
            target: '3 commits',
            repo: 'main',
            time: '2 minutes ago',
            details: ['fix: resolve memory leak in useEffect', 'feat: add dark mode toggle', 'chore: update dependencies']
        },
        {
            id: 2,
            user: { name: 'Alex Kumar', avatar: 'AK', color: 'bg-blue-500!' },
            action: 'opened',
            target: 'pull request #142',
            repo: 'feature/auth',
            time: '15 minutes ago',
            description: 'Implement OAuth2 authentication flow'
        },
        {
            id: 3,
            user: { name: 'Maya Johnson', avatar: 'MJ', color: 'bg-emerald-500!' },
            action: 'commented on',
            target: 'issue #89',
            time: '1 hour ago',
            description: "I've investigated this bug and found the root cause. Working on a fix now."
        },
        {
            id: 4,
            user: { name: 'David Park', avatar: 'DP', color: 'bg-amber-500!' },
            action: 'merged',
            target: 'pull request #138',
            repo: 'main',
            time: '3 hours ago'
        },
        {
            id: 5,
            user: { name: 'Emma Wilson', avatar: 'EW', color: 'bg-rose-500!' },
            action: 'created',
            target: 'release v2.4.0',
            time: '5 hours ago',
            description: 'Performance improvements and bug fixes'
        }
    ];
}
```

## Alignment

Content location relative the line is defined with the align property.

```typescript
import { Component } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';

@Component({
    template: `
        <div class="flex flex-wrap gap-12">
            <p-timeline [value]="events" class="w-full md:w-80">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event.status }}</div>
                </ng-template>
            </p-timeline>
            <p-timeline [value]="events" class="w-full md:w-80" align="right">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event.status }}</div>
                </ng-template>
            </p-timeline>
            <p-timeline [value]="events" class="w-full md:w-80" align="alternate">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event.status }}</div>
                </ng-template>
            </p-timeline>
        </div>
    `,
    standalone: true,
    imports: [TimelineModule]
})
export class TimelineAlignmentDemo {
    events = [{ status: 'Ordered' }, { status: 'Processing' }, { status: 'Shipped' }, { status: 'Delivered' }];
}
```

## Basic

Timeline receives the events with the value property as a collection of arbitrary objects. In addition, content template is required to display the representation of an event. Example below is a sample events array that is used throughout the documentation.

```typescript
import { Component } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';

@Component({
    template: `
        <p-timeline [value]="events">
            <ng-template #content let-event>
                <div class="text-sm leading-4">{{ event.status }}</div>
            </ng-template>
        </p-timeline>
    `,
    standalone: true,
    imports: [TimelineModule]
})
export class TimelineBasicDemo {
    events = [{ status: 'Ordered' }, { status: 'Processing' }, { status: 'Shipped' }, { status: 'Delivered' }];
}
```

## Custom

Sample implementation with custom content, styled markers, and rich event cards.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { TimelineModule } from 'primeng/timeline';
import { ButtonModule } from 'primeng/button';
import { Box } from '@primeicons/angular/box';
import { MapMarker } from '@primeicons/angular/map-marker';
import { PIcon } from '@primeicons/angular/p-icon';

@Component({
    template: `
        <p-timeline
            [value]="events"
            align="alternate"
            class="w-full @container"
            [pt]="{ event: { class: '@max-[480px]:flex-row!' }, eventOpposite: { class: '@max-[480px]:hidden' }, eventContent: { class: '@max-[480px]:text-left!' }, eventConnector: { class: 'mb-4' } }"
        >
            <ng-template #opposite let-event>
                <div class="font-medium text-surface-700 dark:text-surface-200">{{ event.date }}</div>
                <div class="text-sm text-surface-500 dark:text-surface-400">{{ event.time }}</div>
            </ng-template>
            <ng-template #marker let-event>
                <span [class]="'flex items-center justify-center w-12 h-12 rounded-full text-white shadow-lg ' + event.color">
                    <svg [pIcon]="event.icon" class="text-lg" />
                </span>
            </ng-template>
            <ng-template #content let-event>
                <div class="p-5 rounded-xl bg-surface-0 dark:bg-surface-800 border border-surface-200 dark:border-surface-700 shadow-sm mb-4 text-left">
                    <div class="hidden @max-[480px]:block mb-2 text-sm text-surface-500 dark:text-surface-400">{{ event.date }} · {{ event.time }}</div>
                    <div class="flex items-center gap-3 mb-3">
                        <p-avatar [label]="event.user" shape="circle" class="bg-primary/10! text-primary! font-semibold!" />
                        <span class="font-bold text-surface-900 dark:text-surface-0">{{ event.status }}</span>
                    </div>
                    <span class="text-surface-600 dark:text-surface-300 text-sm leading-relaxed">{{ event.description }}</span>
                    @if (event.details) {
                        <ul class="mt-3 space-y-1">
                            @for (detail of event.details; track detail) {
                                <li class="text-sm text-surface-500 dark:text-surface-400 flex items-center gap-2">
                                    <svg data-p-icon="box" class="text-xs" />
                                    {{ detail }}
                                </li>
                            }
                        </ul>
                    }
                    @if (event.tracking) {
                        <div class="mt-4 p-3 rounded-lg bg-surface-100 dark:bg-surface-700 flex items-center justify-between">
                            <span class="text-sm text-surface-600 dark:text-surface-300 flex items-center">
                                <svg data-p-icon="map-marker" class="mr-2 shrink-0" />
                                Tracking: {{ event.tracking }}
                            </span>
                            <button type="button" pButton variant="text" size="small">Track</button>
                        </div>
                    }
                </div>
            </ng-template>
        </p-timeline>
    `,
    standalone: true,
    imports: [AvatarModule, TimelineModule, ButtonModule, Box, MapMarker, PIcon],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TimelineCustomDemo {
    events = [
        {
            status: 'Order Placed',
            date: 'Oct 15, 2026',
            time: '10:30 AM',
            icon: 'shopping-cart',
            color: 'bg-blue-500',
            user: 'JD',
            description: 'Your order #12345 has been confirmed and is being prepared for processing.',
            details: ['2x Wireless Headphones', '1x Phone Case', '1x USB-C Cable']
        },
        {
            status: 'Payment Confirmed',
            date: 'Oct 15, 2026',
            time: '10:32 AM',
            icon: 'credit-card',
            color: 'bg-green-500',
            user: 'SY',
            description: 'Payment of $149.99 was successfully processed via Credit Card ending in 4242.'
        },
        {
            status: 'Shipped',
            date: 'Oct 16, 2026',
            time: '02:15 PM',
            icon: 'truck',
            color: 'bg-orange-500',
            user: 'MK',
            description: 'Package has been handed to the carrier and is on its way.',
            tracking: 'TRK-892374651'
        },
        {
            status: 'Delivered',
            date: 'Oct 18, 2026',
            time: '11:20 AM',
            icon: 'check-circle',
            color: 'bg-lime-500',
            user: 'JD',
            description: 'Package was delivered and signed for at the front door.'
        }
    ];
}
```

## Horizontal

TimeLine orientation is controlled with the layout property, default is vertical having horizontal as the alternative.

```typescript
import { Component } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';

@Component({
    template: `
        <div class="flex flex-col gap-4">
            <p-timeline [value]="events" layout="horizontal" align="top">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event }}</div>
                </ng-template>
            </p-timeline>
            <p-timeline [value]="events" layout="horizontal" align="bottom">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event }}</div>
                </ng-template>
            </p-timeline>
            <p-timeline [value]="events" layout="horizontal" align="alternate">
                <ng-template #content let-event>
                    <div class="text-sm leading-4">{{ event }}</div>
                </ng-template>
                <ng-template #opposite let-event><span>&nbsp;</span></ng-template>
            </p-timeline>
        </div>
    `,
    standalone: true,
    imports: [TimelineModule]
})
export class TimelineHorizontalDemo {
    events = ['2026', '2027', '2028', '2029'];
}
```

## Interactive

Build interactive step-based workflows with progress tracking and state management.

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';
import { ButtonModule } from 'primeng/button';
import { Refresh } from '@primeicons/angular/refresh';
import { Check } from '@primeicons/angular/check';
import { CheckCircle } from '@primeicons/angular/check-circle';
import { PIcon } from '@primeicons/angular/p-icon';

interface Step {
    id: number;
    label: string;
    icon: string;
}

@Component({
    template: `
        <div class="flex flex-col gap-6">
            <div class="flex items-center justify-between">
                <div>
                    <h3 class="text-lg font-semibold m-0">Onboarding Progress</h3>
                    <p class="text-surface-500 text-sm mt-1 mb-0">{{ completedSteps.length }} of {{ allSteps.length }} steps completed</p>
                </div>
                <button pButton [variant]="'outlined'" [size]="'small'" (click)="handleReset()">
                    <svg data-p-icon="refresh" />
                    Reset
                </button>
            </div>
            <div class="w-full bg-surface-200 dark:bg-surface-700 rounded-full h-2">
                <div class="bg-primary rounded-full h-2 transition-all duration-500" [style.width.%]="(completedSteps.length / allSteps.length) * 100"></div>
            </div>
            <p-timeline [value]="allSteps" [pt]="{ eventContent: { class: 'min-w-0' } }">
                <ng-template #marker let-step>
                    <button
                        type="button"
                        [class]="
                            'w-10 h-10 rounded-full flex items-center justify-center border-none transition-all duration-300 ' +
                            (getStepStatus(step.id) === 'completed'
                                ? 'bg-green-500 text-white cursor-default'
                                : getStepStatus(step.id) === 'current'
                                  ? 'bg-primary text-primary-contrast cursor-pointer hover:scale-110 animate-pulse'
                                  : 'bg-surface-200 dark:bg-surface-700 text-surface-400 cursor-not-allowed')
                        "
                        (click)="handleComplete(step.id)"
                        [disabled]="getStepStatus(step.id) !== 'current'"
                    >
                        @if (getStepStatus(step.id) === 'completed') {
                            <svg data-p-icon="check" />
                        } @else {
                            <svg [pIcon]="step.icon" />
                        }
                    </button>
                </ng-template>
                <ng-template #content let-step>
                    <div [class]="'p-3 rounded-lg transition-all duration-300 ' + (getStepStatus(step.id) === 'completed' ? 'bg-green-50 dark:bg-green-900/20' : getStepStatus(step.id) === 'current' ? 'bg-primary/10' : 'opacity-50')">
                        <p [class]="'m-0 font-medium ' + (getStepStatus(step.id) === 'completed' ? 'text-green-700 dark:text-green-400 line-through' : getStepStatus(step.id) === 'current' ? 'text-primary' : 'text-surface-500')">
                            {{ step.label }}
                        </p>
                        @if (getStepStatus(step.id) === 'current') {
                            <p class="text-xs text-surface-500 mt-1 mb-0">Click the marker to complete</p>
                        }
                    </div>
                </ng-template>
            </p-timeline>
            @if (completedSteps.length === allSteps.length) {
                <div class="p-4 rounded-lg bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 flex flex-col items-center gap-2">
                    <svg data-p-icon="check-circle" size="24 " class="text-green-500" />
                    <div class="font-semibold text-green-700 dark:text-green-400">Onboarding Complete!</div>
                    <span class="text-sm text-green-600 dark:text-green-500 leading-none">You've completed all the steps.</span>
                </div>
            }
        </div>
    `,
    standalone: true,
    imports: [TimelineModule, ButtonModule, Refresh, Check, CheckCircle, PIcon],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TimelineInteractiveDemo {
    allSteps: Step[] = [
        { id: 1, label: 'Account Created', icon: 'user-plus' },
        { id: 2, label: 'Email Verified', icon: 'envelope' },
        { id: 3, label: 'Profile Completed', icon: 'id-card' },
        { id: 4, label: 'First Purchase', icon: 'shopping-bag' },
        { id: 5, label: 'Review Posted', icon: 'star' }
    ];

    completedSteps: number[] = [1];

    currentStep: number = 2;

    handleComplete(stepId: number): void {
        if (stepId === this.currentStep) {
            this.completedSteps = [...this.completedSteps, stepId];
            this.currentStep++;
        }
    }

    handleReset(): void {
        this.completedSteps = [1];
        this.currentStep = 2;
    }

    getStepStatus(stepId: number): 'completed' | 'current' | 'pending' {
        if (this.completedSteps.includes(stepId)) {
            return 'completed';
        }

        if (stepId === this.currentStep) {
            return 'current';
        }

        return 'pending';
    }
}
```

## Opposite

Additional content at the other side of the line can be provided with the opposite property.

```typescript
import { Component } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';

@Component({
    template: `
        <p-timeline [value]="events">
            <ng-template #opposite let-event>
                <div class="text-xs leading-4 text-surface-500 dark:text-surface-400">{{ event.date }}</div>
            </ng-template>
            <ng-template #content let-event>
                <div class="text-sm leading-4">{{ event.status }}</div>
            </ng-template>
        </p-timeline>
    `,
    standalone: true,
    imports: [TimelineModule]
})
export class TimelineOppositeDemo {
    events = [
        { status: 'Ordered', date: '15/10/2026 10:30' },
        { status: 'Processing', date: '15/10/2026 14:00' },
        { status: 'Shipped', date: '15/10/2026 16:15' },
        { status: 'Delivered', date: '16/10/2026 10:00' }
    ];
}
```

## preview-doc

```typescript
import { Component, ChangeDetectionStrategy } from '@angular/core';
import { TimelineModule } from 'primeng/timeline';

@Component({
    template: `
        <p-timeline [value]="events">
            <ng-template #content let-event>
                <div class="text-sm leading-4">{{ event.status }}</div>
            </ng-template>
        </p-timeline>
    `,
    standalone: true,
    imports: [TimelineModule],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TimelinePreviewDemo {
    events = [{ status: 'Ordered' }, { status: 'Processing' }, { status: 'Shipped' }, { status: 'Delivered' }];
}
```

## template-doc

Sample implementation with custom content and styled markers.

```typescript
import { Component } from '@angular/core';
import { CardModule } from 'primeng/card';
import { TimelineModule } from 'primeng/timeline';
import { ButtonModule } from 'primeng/button';

interface EventItem {
    status?: string;
    date?: string;
    icon?: string;
    color?: string;
    image?: string;
}

@Component({
    template: `
        <p-timeline [value]="events" align="alternate" class="customized-timeline">
            <ng-template #marker let-event>
                <span class="flex w-8 h-8 items-center justify-center text-white rounded-full z-10 shadow-xs" [style]="{ 'background-color': event.color }">
                    <i [class]="event.icon"></i>
                </span>
            </ng-template>
            <ng-template #content let-event>
                <p-card [header]="event.status" [subheader]="event.date">
                    @if (event.image) {
                        <img [src]="'https://primefaces.org/cdn/primeng/images/demo/product/' + event.image" [alt]="event.name" width="200" class="shadow-sm" />
                    }
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipisicing elit. Inventore sed consequuntur error repudiandae numquam deserunt quisquam repellat libero asperiores earum nam nobis, culpa ratione quam perferendis esse, cupiditate
                        neque quas!
                    </p>
                    <button type="button" pButton [text]="true">Read more</button>
                </p-card>
            </ng-template>
        </p-timeline>
    `,
    standalone: true,
    imports: [CardModule, TimelineModule, ButtonModule]
})
export class TimelineTemplateDemo {
    events: EventItem[];

    constructor() {
        this.events = [
            { status: 'Ordered', date: '15/10/2020 10:30', icon: 'pi pi-shopping-cart', color: '#9C27B0', image: 'game-controller.jpg' },
            { status: 'Processing', date: '15/10/2020 14:00', icon: 'pi pi-cog', color: '#673AB7' },
            { status: 'Shipped', date: '15/10/2020 16:15', icon: 'pi pi-shopping-cart', color: '#FF9800' },
            { status: 'Delivered', date: '16/10/2020 10:00', icon: 'pi pi-check', color: '#607D8B' }
        ];
    }
}
```

## Timeline

Timeline visualizes a series of chained events.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, TimelinePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | any[] \| undefined | - | An array of events to display. |
| align | "left" \| "right" \| "top" \| "bottom" \| "alternate" | - | Position of the timeline bar relative to the content. Valid values are "left", "right" for vertical layout and "top", "bottom" for horizontal layout. |
| layout | "vertical" \| "horizontal" | - | Orientation of the timeline. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| content | TemplateRef<TimelineItemTemplateContext<any>> \| undefined | Custom content template. |
| opposite | TemplateRef<TimelineItemTemplateContext<any>> \| undefined | Custom opposite item template. |
| marker | TemplateRef<TimelineItemTemplateContext<any>> \| undefined | Custom marker template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| event | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event's DOM element. |
| eventOpposite | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event opposite's DOM element. |
| eventSeparator | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event separator's DOM element. |
| eventMarker | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event marker's DOM element. |
| eventConnector | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event connector's DOM element. |
| eventContent | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the event content's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-timeline | Class name of the root element |
| p-timeline-event | Class name of the event element |
| p-timeline-event-opposite | Class name of the event opposite element |
| p-timeline-event-separator | Class name of the event separator element |
| p-timeline-event-marker | Class name of the event marker element |
| p-timeline-event-connector | Class name of the event connector element |
| p-timeline-event-content | Class name of the event content element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| timeline.event.min.height | --p-timeline-event-min-height | Min height of event |
| timeline.horizontal.event.content.padding | --p-timeline-horizontal-event-content-padding | Event content padding of horizontal |
| timeline.vertical.event.content.padding | --p-timeline-vertical-event-content-padding | Event content padding of vertical |
| timeline.event.marker.size | --p-timeline-event-marker-size | Size of event marker |
| timeline.event.marker.border.radius | --p-timeline-event-marker-border-radius | Border radius of event marker |
| timeline.event.marker.border.width | --p-timeline-event-marker-border-width | Border width of event marker |
| timeline.event.marker.background | --p-timeline-event-marker-background | Background of event marker |
| timeline.event.marker.border.color | --p-timeline-event-marker-border-color | Border color of event marker |
| timeline.event.marker.content.border.radius | --p-timeline-event-marker-content-border-radius | Content border radius of event marker |
| timeline.event.marker.content.size | --p-timeline-event-marker-content-size | Content size of event marker |
| timeline.event.marker.content.background | --p-timeline-event-marker-content-background | Content background of event marker |
| timeline.event.marker.content.inset.shadow | --p-timeline-event-marker-content-inset-shadow | Content inset shadow of event marker |
| timeline.event.connector.color | --p-timeline-event-connector-color | Color of event connector |
| timeline.event.connector.size | --p-timeline-event-connector-size | Size of event connector |
