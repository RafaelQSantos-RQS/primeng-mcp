---
title: "Angular Listbox Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/listbox.md"
description: "Listbox is used to select one or more values from a list of items."
---

# Angular Listbox Component

Listbox is used to select one or more values from a list of items.

## Accessibility

Screen Reader Value to describe the component can be provided ariaLabelledBy or ariaLabel props. The list element has a listbox role with the aria-multiselectable attribute that sets to true when multiple selection is enabled. Each list item has an option role with aria-selected and aria-disabled as their attributes.

## Basic

Select one or multiple options from a scrollable list.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxBasicDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Checkbox

Combine the multiple property with the checkbox property to render checkboxes for each option.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [(ngModel)]="selectedCity" [options]="cities" [multiple]="true" [checkbox]="true" optionLabel="name" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxCheckboxDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Checkmark

Enable the checkmark property to display a checkmark indicator next to the selected option. Set highlightOnSelect to false to use checkmark-only styling.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [(ngModel)]="selectedCity" [options]="cities" optionLabel="name" [checkmark]="true" [highlightOnSelect]="false" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxCheckMarkDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## customgroup-doc

Use the group template to customize group headers with custom content like flags.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { SelectItemGroup } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox
                [options]="groupedCities"
                [group]="true"
                [(ngModel)]="selectedCity"
                optionLabel="label"
                optionValue="value"
                class="w-full md:w-56"
                scrollHeight="250px"
                [pt]="{ list: { class: 'p-0!' }, optionGroup: { class: 'border-b border-surface rounded-none' } }"
            >
                <ng-template let-group #group>
                    <div class="flex items-center gap-2">
                        <span>{{ countryFlags[group.value] }}</span>
                        <span>{{ group.label }}</span>
                    </div>
                </ng-template>
            </p-listbox>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxCustomGroupDemo {
    groupedCities!: SelectItemGroup[];

    selectedCity!: string;

    countryFlags: Record<string, string> = {
        de: '🇩🇪',
        us: '🇺🇸',
        jp: '🇯🇵'
    };

    constructor() {
        this.groupedCities = [
            {
                label: 'Germany',
                value: 'de',
                items: [
                    { label: 'Berlin', value: 'Berlin' },
                    { label: 'Frankfurt', value: 'Frankfurt' },
                    { label: 'Hamburg', value: 'Hamburg' },
                    { label: 'Munich', value: 'Munich' }
                ]
            },
            {
                label: 'USA',
                value: 'us',
                items: [
                    { label: 'Chicago', value: 'Chicago' },
                    { label: 'Los Angeles', value: 'Los Angeles' },
                    { label: 'New York', value: 'New York' },
                    { label: 'San Francisco', value: 'San Francisco' }
                ]
            },
            {
                label: 'Japan',
                value: 'jp',
                items: [
                    { label: 'Kyoto', value: 'Kyoto' },
                    { label: 'Osaka', value: 'Osaka' },
                    { label: 'Tokyo', value: 'Tokyo' },
                    { label: 'Yokohama', value: 'Yokohama' }
                ]
            }
        ];
    }
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" [disabled]="true" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxDisabledDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## dragdrop-doc

Items can be reordered using drag and drop by enabling dragdrop property. Depends on &#64;angular/cdk package.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" [dragdrop]="true" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxDragDropDemo implements OnInit {
    cities!: any[];

    selectedCity!: any;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Filter

ListBox provides built-in filtering that is enabled by adding the filter property.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" [filter]="true" filterPlaceHolder="Search city" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxFilterDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Focus Behavior

Use autoOptionFocus to control initial focused option behavior, selectOnFocus to select options while navigating with focus, and focusOnHover to move focus with mouse hover when the component is focused.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { ButtonModule } from 'primeng/button';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2 justify-center">
                <button pButton [severity]="autoOptionFocus ? 'primary' : 'secondary'" size="small" (click)="autoOptionFocus = !autoOptionFocus">autoOptionFocus: {{ autoOptionFocus }}</button>
                <button pButton [severity]="selectOnFocus ? 'primary' : 'secondary'" size="small" (click)="selectOnFocus = !selectOnFocus">selectOnFocus: {{ selectOnFocus }}</button>
                <button pButton [severity]="focusOnHover ? 'primary' : 'secondary'" size="small" (click)="focusOnHover = !focusOnHover">focusOnHover: {{ focusOnHover }}</button>
            </div>
            <div class="flex justify-center">
                <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" optionValue="code" [autoOptionFocus]="autoOptionFocus" [selectOnFocus]="selectOnFocus" [focusOnHover]="focusOnHover" class="w-full md:w-56" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, ButtonModule, FormsModule]
})
export class ListboxFocusBehaviorDemo implements OnInit {
    cities!: City[];

    selectedCity!: string;

    autoOptionFocus: boolean = true;

    selectOnFocus: boolean = false;

    focusOnHover: boolean = true;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Group

Options can be grouped when a nested data structure is provided by enabling the group property.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { SelectItemGroup } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="groupedCities" [group]="true" [(ngModel)]="selectedCity" optionLabel="label" optionValue="value" class="w-full md:w-56" scrollHeight="250px" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxGroupDemo {
    groupedCities!: SelectItemGroup[];

    selectedCity!: string;

    constructor() {
        this.groupedCities = [
            {
                label: 'Germany',
                value: 'de',
                items: [
                    { label: 'Berlin', value: 'Berlin' },
                    { label: 'Frankfurt', value: 'Frankfurt' },
                    { label: 'Hamburg', value: 'Hamburg' },
                    { label: 'Munich', value: 'Munich' }
                ]
            },
            {
                label: 'USA',
                value: 'us',
                items: [
                    { label: 'Chicago', value: 'Chicago' },
                    { label: 'Los Angeles', value: 'Los Angeles' },
                    { label: 'New York', value: 'New York' },
                    { label: 'San Francisco', value: 'San Francisco' }
                ]
            },
            {
                label: 'Japan',
                value: 'jp',
                items: [
                    { label: 'Kyoto', value: 'Kyoto' },
                    { label: 'Osaka', value: 'Osaka' },
                    { label: 'Tokyo', value: 'Tokyo' },
                    { label: 'Yokohama', value: 'Yokohama' }
                ]
            }
        ];
    }
}
```

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" class="w-full md:w-56" [invalid]="selectedCity === undefined" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxInvalidDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Meta Key Selection

Use metaKeySelection in multiple mode to require Ctrl / Cmd for additive selection.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="space-y-3">
            <p class="text-sm text-muted-color text-center">Hold <kbd class="px-1 py-0.5 border rounded text-xs">Ctrl</kbd> (or <kbd class="px-1 py-0.5 border rounded text-xs">Cmd</kbd> on macOS) while selecting to keep existing selections.</p>
            <div class="flex justify-center">
                <p-listbox [options]="cities" [(ngModel)]="selectedCities" optionLabel="name" [multiple]="true" [metaKeySelection]="true" class="w-full md:w-56" />
            </div>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxMetaKeySelectionDemo implements OnInit {
    cities!: City[];

    selectedCities!: City[];

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Multiple

ListBox allows choosing a single item by default, enable multiple property to choose more than one. When the optional metaKeySelection is present, behavior is changed in a way that selecting a new item requires meta key to be present.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCities" optionLabel="name" [multiple]="true" [metaKeySelection]="false" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxMultipleDemo implements OnInit {
    cities!: City[];

    selectedCities!: City[];

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }
}
```

## Option

For custom content support define a template named item where the default local template variable refers to an option.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { Country } from '@/domain/customer';

interface Country {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="countries" [(ngModel)]="selectedCountry" optionLabel="name" class="w-full md:w-56">
                <ng-template #item let-country>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
            </p-listbox>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxOptionDemo implements OnInit {
    countries!: Country[];

    selectedCountry!: Country;

    ngOnInit() {
        this.countries = [
            { name: 'Australia', code: 'AU' },
            { name: 'Brazil', code: 'BR' },
            { name: 'China', code: 'CN' },
            { name: 'Egypt', code: 'EG' },
            { name: 'France', code: 'FR' },
            { name: 'Germany', code: 'DE' },
            { name: 'India', code: 'IN' },
            { name: 'Japan', code: 'JP' },
            { name: 'Spain', code: 'ES' },
            { name: 'United States', code: 'US' }
        ];
    }
}
```

## Disabled Options

Use optionDisabled to disable specific items by field name while keeping the listbox itself interactive.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface City {
    name: string;
    code: string;
    disabled?: boolean;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" optionDisabled="disabled" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxOptionDisabledDemo implements OnInit {
    cities!: City[];

    selectedCity!: City;

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN', disabled: true },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS', disabled: true }
        ];
    }
}
```

## preview-doc

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

interface Airport {
    name: string;
    code: string;
    city: string;
    country: string;
}

@Component({
    template: `
        <div class="flex flex-col gap-2 w-full md:w-60 mx-auto">
            <div class="flex items-center justify-between">
                <label class="text-sm font-medium">Choose Airport</label>
                <span class="text-xs text-muted-color">{{ airports.length }} airports</span>
            </div>
            <p-listbox [options]="airports" [(ngModel)]="selectedAirport" optionLabel="name" scrollHeight="auto" class="w-full">
                <ng-template #item let-airport>
                    <div class="flex items-center justify-between w-full gap-3">
                        <div class="flex items-center gap-3 min-w-0">
                            <img alt="flag" src="https://primefaces.org/cdn/primevue/images/flag/flag_placeholder.png" [class]="'flag flag-' + airport.country + ' shrink-0'" style="width: 18px; height: 12px" />
                            <span class="truncate">{{ airport.name }}</span>
                        </div>
                        <span class="text-xs font-mono px-1.5 py-0.5 rounded bg-surface-100 dark:bg-surface-800 text-muted-color in-data-selected:bg-primary in-data-selected:text-primary-contrast">{{ airport.code }}</span>
                    </div>
                </ng-template>
            </p-listbox>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxPreviewDemo implements OnInit {
    airports!: Airport[];

    selectedAirport!: Airport;

    ngOnInit() {
        this.airports = [
            { name: 'John F. Kennedy', code: 'JFK', city: 'New York', country: 'us' },
            { name: 'Heathrow', code: 'LHR', city: 'London', country: 'gb' },
            { name: 'Charles de Gaulle', code: 'CDG', city: 'Paris', country: 'fr' },
            { name: 'Frankfurt', code: 'FRA', city: 'Frankfurt', country: 'de' },
            { name: 'Schiphol', code: 'AMS', city: 'Amsterdam', country: 'nl' },
            { name: 'Istanbul', code: 'IST', city: 'Istanbul', country: 'tr' },
            { name: 'Dubai', code: 'DXB', city: 'Dubai', country: 'ae' },
            { name: 'Changi', code: 'SIN', city: 'Singapore', country: 'sg' },
            { name: 'Haneda', code: 'HND', city: 'Tokyo', country: 'jp' }
        ];
    }
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-listbox [options]="cities" formControlName="selectedCity" optionLabel="name" class="w-full md:w-56" [invalid]="isInvalid('selectedCity')" />
                    @if (isInvalid('selectedCity')) {
                        <p-message severity="error" size="small" variant="simple">City is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, MessageModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class ListboxReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    cities!: City[];

    constructor() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
        this.exampleForm = this.fb.group({
            selectedCity: ['', Validators.required]
        });
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        }
    }

    isInvalid(controlName: string) {
        const control = this.exampleForm.get(controlName);

        return control?.invalid && this.formSubmitted;
    }
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { ListboxModule } from 'primeng/listbox';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <p-listbox [options]="cities" [formField]="exampleForm.selectedCity" optionLabel="name" class="w-full md:w-56" />
                    @if (exampleForm.selectedCity().touched() && exampleForm.selectedCity().invalid()) {
                        @for (error of exampleForm.selectedCity().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, MessageModule, ToastModule, ButtonModule, FormField]
})
export class ListboxSignalFormsDemo {
    messageService = inject(MessageService);

    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    model = signal<{ selectedCity: City | null }>({ selectedCity: null });

    exampleForm = form(this.model, (path) => {
        required(path.selectedCity, { when: ({ state }) => state.touched(), message: 'City is required.' });
    });

    onSubmit(event: Event) {
        event.preventDefault();
        submit(this.exampleForm, async () => {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        });
    }
}
```

## Template

For custom content support define a template named item where the default local template variable refers to an option.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { Country } from '@/domain/customer';

interface Country {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox [options]="countries" [(ngModel)]="selectedCountry" optionLabel="name" class="w-full md:w-56">
                <ng-template #item let-country>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
            </p-listbox>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxTemplateDemo implements OnInit {
    countries!: Country[];

    selectedCountry!: Country;

    ngOnInit() {
        this.countries = [
            { name: 'Australia', code: 'AU' },
            { name: 'Brazil', code: 'BR' },
            { name: 'China', code: 'CN' },
            { name: 'Egypt', code: 'EG' },
            { name: 'France', code: 'FR' },
            { name: 'Germany', code: 'DE' },
            { name: 'India', code: 'IN' },
            { name: 'Japan', code: 'JP' },
            { name: 'Spain', code: 'ES' },
            { name: 'United States', code: 'US' }
        ];
    }
}
```

## templatedrivenforms-doc

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex justify-center flex-col gap-4 md:w-56">
                <div class="flex flex-col gap-1">
                    <p-listbox #city="ngModel" [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" class="w-full md:w-56" [invalid]="city.invalid && exampleForm.submitted" name="city" required />
                    @if (city.invalid && exampleForm.submitted) {
                        <p-message severity="error" size="small" variant="simple">City is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, MessageModule, ToastModule, ButtonModule, FormsModule]
})
export class ListboxTemplateDrivenFormsDemo implements OnInit {
    messageService = inject(MessageService);

    selectedCity!: City;

    cities!: City[];

    ngOnInit() {
        this.cities = [
            { name: 'New York', code: 'NY' },
            { name: 'Rome', code: 'RM' },
            { name: 'London', code: 'LDN' },
            { name: 'Istanbul', code: 'IST' },
            { name: 'Paris', code: 'PRS' }
        ];
    }

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## Virtual Scroll

VirtualScrolling is an efficient way of rendering the options by displaying a small subset of data in the viewport at any time. When dealing with huge number of options, it is suggested to enable VirtualScrolling to avoid performance issues. Usage is simple as setting virtualScroll property to true and defining virtualScrollItemSize to specify the height of an item.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ListboxModule } from 'primeng/listbox';

@Component({
    template: `
        <div class="flex justify-center">
            <p-listbox
                [options]="items"
                [(ngModel)]="selectedItems"
                [selectAll]="selectAll"
                [filter]="true"
                [checkbox]="true"
                optionLabel="label"
                [virtualScroll]="true"
                [virtualScrollItemSize]="40"
                [multiple]="true"
                [metaKeySelection]="false"
                (onSelectAllChange)="onSelectAllChange($event)"
                (onChange)="onChange($event)"
                scrollHeight="250px"
                [striped]="true"
                class="w-full md:w-56"
            />
        </div>
    `,
    standalone: true,
    imports: [ListboxModule, FormsModule]
})
export class ListboxVirtualScrollDemo {
    items = Array.from({ length: 100000 }, (_, i) => ({ label: `Item #${i}`, value: i }));

    selectedItems!: any[];

    selectAll: boolean = false;

    onSelectAllChange(event) {
        this.selectedItems = event.checked ? [...this.items] : [];
        this.selectAll = event.checked;
    }

    onChange(event) {
        const { value } = event;

        if (value) this.selectAll = value.length === this.items.length;
    }
}
```

## Listbox

ListBox is used to select one or more values from a list of items.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ListBoxPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| id | string \| undefined | - | Unique identifier of the component. |
| searchMessage | string \| undefined | '{0} results are available' | Text to display when the search is active. Defaults to global value in i18n translation configuration. |
| emptySelectionMessage | string \| undefined | 'No selected item' | Text to display when filtering does not return any results. Defaults to global value in i18n translation configuration. |
| selectionMessage | string \| undefined | '{0} items selected' | Text to be displayed in hidden accessible field when options are selected. Defaults to global value in i18n translation configuration. |
| autoOptionFocus | boolean | - | Whether to focus on the first visible or selected element when the overlay panel is shown. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| selectOnFocus | boolean \| undefined | - | When enabled, the focused option is selected. |
| searchLocale | boolean \| undefined | - | Locale to use in searching. The default locale is the host environment's current locale. |
| focusOnHover | boolean | - | When enabled, the hovered option will be focused. |
| filterMessage | string \| undefined | - | Text to display when filtering. |
| filterFields | any[] \| undefined | - | Fields used when filtering the options, defaults to optionLabel. |
| lazy | boolean | - | Defines if data is loaded and interacted with in lazy manner. |
| virtualScroll | boolean \| undefined | - | Whether the data should be loaded on demand during scroll. |
| virtualScrollItemSize | number \| undefined | - | Height of an item in the list for VirtualScrolling. |
| virtualScrollOptions | ScrollerOptions \| undefined | - | Whether to use the scroller feature. The properties of scroller component can be used like an object in it. |
| scrollHeight | string | - | Height of the viewport in pixels, a scrollbar is defined if height of list exceeds this value. |
| tabindex | number | - | Index of the element in tabbing order. |
| multiple | boolean \| undefined | - | When specified, allows selecting multiple values. |
| listStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the list element. |
| listStyleClass | string \| undefined | - | Style class of the list element. |
| readonly | boolean \| undefined | - | When present, it specifies that the element value cannot be changed. |
| checkbox | boolean | - | When specified, allows selecting items with checkboxes. |
| filter | boolean | - | When specified, displays a filter input at header. |
| filterBy | string \| undefined | - | When filtering is enabled, filterBy decides which field or fields (comma separated) to search against. |
| filterMatchMode | "startsWith" \| "contains" \| "notContains" \| "endsWith" \| "equals" \| "notEquals" \| "in" \| "between" \| "lt" \| "lte" \| "gt" \| "gte" \| "is" \| "isNot" \| "before" \| "after" \| "dateIs" \| "dateIsNot" \| "dateBefore" \| "dateAfter" | - | Defines how the items are filtered. |
| filterLocale | string \| undefined | - | Locale to use in filtering. The default locale is the host environment's current locale. |
| metaKeySelection | boolean | - | Defines how multiple items can be selected, when true metaKey needs to be pressed to select or unselect an item and when set to false selection of each item can be toggled individually. On touch enabled devices, metaKeySelection is turned off automatically. |
| dataKey | string \| undefined | - | A property to uniquely identify a value in options. |
| showToggleAll | boolean | - | Whether header checkbox is shown in multiple mode. |
| optionLabel | string \| undefined | - | Name of the label field of an option. |
| optionValue | string \| undefined | - | Name of the value field of an option. |
| optionGroupChildren | string | - | Name of the options field of an option group. |
| optionGroupLabel | string | - | Name of the label field of an option group. |
| optionDisabled | string \| ((item: any) => boolean) \| undefined | - | Name of the disabled field of an option or function to determine disabled state. |
| ariaFilterLabel | string \| undefined | - | Defines a string that labels the filter input. |
| filterPlaceHolder | string \| undefined | - | Defines placeholder of the filter input. |
| emptyFilterMessage | string \| undefined | - | Text to display when filtering does not return any results. |
| emptyMessage | string \| undefined | - | Text to display when there is no data. Defaults to global value in i18n translation configuration. |
| group | boolean \| undefined | - | Whether to display options as grouped when nested options are provided. |
| options | any[] \| undefined | - | An array of selectitems to display as the available options. |
| filterValue | string \| undefined | - | When specified, filter displays with this value. |
| selectAll | boolean \| undefined | - | Whether all data is selected. |
| striped | boolean | false | Whether to displays rows with alternating colors. |
| highlightOnSelect | boolean | true | Whether the selected option will be add highlight class. |
| checkmark | boolean | false | Whether the selected option will be shown with a check mark. |
| dragdrop | boolean | - | Whether to enable dragdrop based reordering. |
| dropListData | any[] \| undefined | - | Array to use for CDK drop list data binding. When not provided, uses options array. |
| fluid | boolean \| undefined | undefined | Spans 100% width of the container when enabled. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: ListboxChangeEvent | Callback to invoke on value change. |
| onClick | event: ListboxClickEvent | Callback to invoke when option is clicked. |
| onDblClick | event: ListboxDoubleClickEvent | Callback to invoke when option is double clicked. |
| onFilter | event: ListboxFilterEvent | Callback to invoke when data is filtered. |
| onFocus | event: FocusEvent | Callback to invoke when component receives focus. |
| onBlur | event: FocusEvent | Callback to invoke when component loses focus. |
| onSelectAllChange | event: ListboxSelectAllChangeEvent | Callback to invoke when all data is selected. |
| onLazyLoad | event: ScrollerLazyLoadEvent | Emits on lazy load. |
| onDrop | value: CdkDragDrop<string[], string[], any | Emits on item is dropped. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<ListboxItemTemplateContext<any>> \| undefined | Custom item template. |
| group | TemplateRef<ListboxGroupTemplateContext<any>> \| undefined | Custom group template. |
| header | TemplateRef<ListboxHeaderTemplateContext<any>> \| undefined | Custom header template. |
| filter | TemplateRef<ListboxFilterTemplateContext> \| undefined | Custom filter template. |
| footer | TemplateRef<ListboxFooterTemplateContext<any>> \| undefined | Custom footer template. |
| emptyfilter | TemplateRef<void> \| undefined | Custom empty filter message template. |
| empty | TemplateRef<void> \| undefined | Custom empty message template. |
| filtericon | TemplateRef<void> \| undefined | Custom filter icon template. |
| checkicon | TemplateRef<ListboxCheckIconTemplateContext> \| undefined | Custom check icon template. |
| checkmark | TemplateRef<ListboxCheckmarkTemplateContext> \| undefined | Custom checkmark icon template. |
| loader | TemplateRef<ListboxLoaderTemplateContext> \| undefined | Custom loader template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| updateModel | value: any, event: any | void | Updates the model value. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| pcCheckbox | any | Used to pass attributes to the Checkbox component. |
| pcFilterContainer | any | Used to pass attributes to the IconField component. |
| pcFilter | any | Used to pass attributes to the filter input's DOM element. |
| pcFilterIconContainer | any | Used to pass attributes to the InputIcon component. |
| filterIcon | PassThroughOption<SVGElement, I> | Used to pass attributes to the filter icon's DOM element. |
| hiddenFilterResult | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden filter result's DOM element. |
| listContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the list container's DOM element. |
| virtualScroller | any | Used to pass attributes to the VirtualScroller component. |
| list | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the list's DOM element. |
| optionGroup | PassThroughOption<HTMLLIElement, I, ListBoxContext> | Used to pass attributes to the option group's DOM element. |
| option | PassThroughOption<HTMLLIElement, I, ListBoxContext> | Used to pass attributes to the option's DOM element. |
| optionCheckIcon | PassThroughOption<SVGElement, I> | Used to pass attributes to the option check icon's DOM element. |
| optionBlankIcon | PassThroughOption<SVGElement, I> | Used to pass attributes to the option blank icon's DOM element. |
| emptyMessage | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the empty message's DOM element. |
| hiddenEmptyMessage | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden empty message's DOM element. |
| hiddenSelectedMessage | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden selected message's DOM element. |
| hiddenFirstFocusableEl | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the first hidden focusable element. |
| hiddenLastFocusableEl | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the last hidden focusable element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-listbox | Class name of the root element |
| p-listbox-header | Class name of the header element |
| p-listbox-filter | Class name of the filter element |
| p-listbox-list-container | Class name of the list container element |
| p-listbox-list | Class name of the list element |
| p-listbox-option-group | Class name of the option group element |
| p-listbox-option | Class name of the option element |
| p-listbox-option-check-icon | Class name of the option check icon element |
| p-listbox-option-blank-icon | Class name of the option blank icon element |
| p-listbox-empty-message | Class name of the empty message element |
| p-listbox-drag-preview | Class name of the multi-selection drag preview container |
| p-listbox-drag-preview-more | Class name of the overflow row in the multi-selection drag preview |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| listbox.background | --p-listbox-background | Background of root |
| listbox.disabled.background | --p-listbox-disabled-background | Disabled background of root |
| listbox.border.color | --p-listbox-border-color | Border color of root |
| listbox.invalid.border.color | --p-listbox-invalid-border-color | Invalid border color of root |
| listbox.color | --p-listbox-color | Color of root |
| listbox.disabled.color | --p-listbox-disabled-color | Disabled color of root |
| listbox.shadow | --p-listbox-shadow | Shadow of root |
| listbox.border.radius | --p-listbox-border-radius | Border radius of root |
| listbox.transition.duration | --p-listbox-transition-duration | Transition duration of root |
| listbox.list.padding | --p-listbox-list-padding | Padding of list |
| listbox.list.gap | --p-listbox-list-gap | Gap of list |
| listbox.list.header.padding | --p-listbox-list-header-padding | Header padding of list |
| listbox.option.focus.background | --p-listbox-option-focus-background | Focus background of option |
| listbox.option.selected.background | --p-listbox-option-selected-background | Selected background of option |
| listbox.option.selected.focus.background | --p-listbox-option-selected-focus-background | Selected focus background of option |
| listbox.option.color | --p-listbox-option-color | Color of option |
| listbox.option.focus.color | --p-listbox-option-focus-color | Focus color of option |
| listbox.option.selected.color | --p-listbox-option-selected-color | Selected color of option |
| listbox.option.selected.focus.color | --p-listbox-option-selected-focus-color | Selected focus color of option |
| listbox.option.selected.font.weight | --p-listbox-option-selected-font-weight | Font weight of a selected option |
| listbox.option.padding | --p-listbox-option-padding | Padding of option |
| listbox.option.border.radius | --p-listbox-option-border-radius | Border radius of option |
| listbox.option.striped.background | --p-listbox-option-striped-background | Striped background of option |
| listbox.option.font.weight | --p-listbox-option-font-weight | Font weight of option |
| listbox.option.font.size | --p-listbox-option-font-size | Font size of option |
| listbox.option.group.background | --p-listbox-option-group-background | Background of option group |
| listbox.option.group.color | --p-listbox-option-group-color | Color of option group |
| listbox.option.group.font.weight | --p-listbox-option-group-font-weight | Font weight of option group |
| listbox.option.group.font.size | --p-listbox-option-group-font-size | Font size of option group |
| listbox.option.group.padding | --p-listbox-option-group-padding | Padding of option group |
| listbox.checkmark.color | --p-listbox-checkmark-color | Color of checkmark |
| listbox.checkmark.gutter.start | --p-listbox-checkmark-gutter-start | Gutter start of checkmark |
| listbox.checkmark.gutter.end | --p-listbox-checkmark-gutter-end | Gutter end of checkmark |
| listbox.empty.message.padding | --p-listbox-empty-message-padding | Padding of empty message |
