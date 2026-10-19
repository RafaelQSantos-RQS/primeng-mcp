---
title: "Angular Select Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/select.md"
description: "Select is used to choose an item from a collection of options."
---

# Angular Select Component

Select is used to choose an item from a collection of options.

## Accessibility

Screen Reader Value to describe the component can either be provided with ariaLabelledBy or ariaLabel props. The select element has a combobox role in addition to aria-haspopup and aria-expanded attributes. If the editable option is enabled aria-autocomplete is also added. The relation between the combobox and the popup is created with aria-controls and aria-activedescendant attribute is used to instruct screen reader which option to read during keyboard navigation within the popup list. The popup list has an id that refers to the aria-controls attribute of the combobox element and uses listbox as the role. Each list item has an option role, an id to match the aria-activedescendant of the input element along with aria-label , aria-selected and aria-disabled attributes. If filtering is enabled, filterInputProps can be defined to give aria-* props to the filter input element.

## Basic

Select is used as a controlled component with ngModel property along with an options collection. Label and value of an option are defined with the optionLabel and optionValue properties respectively. Note that, when options are simple primitive values such as a string array, no optionLabel and optionValue would be necessary.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface Language {
    label: string;
    value: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="languages" [(ngModel)]="selectedLanguage" optionLabel="label" optionValue="value" placeholder="Select a language" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectBasicDemo {
    languages: Language[] = [
        { label: 'English', value: 'en' },
        { label: 'Deutsch', value: 'de' },
        { label: 'Español', value: 'es' },
        { label: 'Français', value: 'fr' },
        { label: 'Italiano', value: 'it' },
        { label: 'Türkçe', value: 'tr' },
        { label: '日本語', value: 'ja' },
        { label: '中文', value: 'zh' }
    ];

    selectedLanguage: string | undefined;
}
```

## checkboxselection-doc

Multiple selection with checkboxes using multiple and custom #item and #header templates.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="cities" [(ngModel)]="selectedCities" [multiple]="true" optionLabel="name" optionValue="code" [showClear]="true" placeholder="Select Cities" class="w-full md:w-56">
                <ng-template #header let-headerClass="class">
                    <div [class]="headerClass">
                        <p-checkbox [ngModel]="allSelected" [binary]="true" [indeterminate]="indeterminate" (ngModelChange)="onToggleAll($event)" label="Select All"/>
                    </div>
                </ng-template>
                <ng-template #selectedItem>
                    <span>{{ getLabel() }}</span>
                </ng-template>
                <ng-template let-city #item>
                    <div class="flex items-center gap-2">
                        <p-checkbox [ngModel]="isItemSelected(city)" [binary]="true" [tabindex]="-1" [readonly]="true" />
                        <span>{{ city.name }}</span>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, SelectModule, FormsModule]
})
export class SelectCheckboxSelectionDemo {
    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    selectedCities: string[] = [];

    getLabel(): string {
        if (this.selectedCities.length === 0) return '';

        const first = this.cities.find((c) => c.code === this.selectedCities[0])?.name ?? this.selectedCities[0];

        return this.selectedCities.length > 1 ? `${first} (+${this.selectedCities.length - 1} more)` : first;
    }

    get allSelected(): boolean {
        return this.selectedCities.length === this.cities.length;
    }

    get indeterminate(): boolean {
        return this.selectedCities.length > 0 && !this.allSelected;
    }

    isItemSelected(city: City): boolean {
        return this.selectedCities.includes(city.code);
    }

    onToggleAll(checked: boolean) {
        this.selectedCities = checked ? this.cities.map((c) => c.code) : [];
    }
}
```

## Checkmark

An alternative way to highlight the selected option is displaying a checkmark instead.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface Theme {
    label: string;
    value: string;
    icon: string;
    description: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="themes" [(ngModel)]="selectedTheme" [checkmark]="true" optionLabel="label" placeholder="Select a Theme" scrollHeight="auto" class="w-full md:w-64" [pt]="{ option: { class: 'flex-row-reverse justify-between' } }">
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex items-center gap-2">
                        <span>{{ selectedOption.icon }}</span>
                        <span>{{ selectedOption.label }}</span>
                    </div>
                </ng-template>
                <ng-template let-theme #item>
                    <div class="flex items-center gap-3 py-1">
                        <span class="text-xl">{{ theme.icon }}</span>
                        <div class="flex flex-col">
                            <span class="font-medium">{{ theme.label }}</span>
                            <span class="text-xs text-surface-500">{{ theme.description }}</span>
                        </div>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectCheckMarkDemo {
    themes: Theme[] = [
        { label: 'Light', value: 'light', icon: '☀️', description: 'Clean and bright interface' },
        { label: 'Dark', value: 'dark', icon: '🌙', description: 'Easy on the eyes' },
        { label: 'System', value: 'system', icon: '💻', description: 'Match your device settings' },
        { label: 'High Contrast', value: 'high-contrast', icon: '◐', description: 'Maximum readability' }
    ];

    selectedTheme: Theme | undefined;
}
```

## Chips

Selected items displayed as chips using a custom #selectedItem template in multiple mode.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ChipModule } from 'primeng/chip';
import { SelectModule } from 'primeng/select';

interface Member {
    name: string;
    code: string;
    avatar: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="members" [(ngModel)]="selected" [multiple]="true" optionLabel="name" optionValue="code" [showClear]="true" placeholder="Select Members" class="w-full">
                <ng-template let-member #item>
                    <div class="flex items-center gap-2">
                        <img [src]="member.avatar" [alt]="member.name" class="w-7 h-7 rounded-full" />
                        <span>{{ member.name }}</span>
                    </div>
                </ng-template>
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex flex-wrap gap-1">
                        @for (code of selected; track code) {
                            <p-chip
                                [label]="getFirstName(code)"
                                [image]="getAvatar(code)"
                                [removable]="true"
                                (onRemove)="removeItem($event, code)"
                                [pt]="{ root: { class: 'py-0!' }, label: { class: 'text-xs!' }, image: { class: 'w-5! h-5!' } }"
                            />
                        }
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [ChipModule, SelectModule, FormsModule]
})
export class SelectChipDemo {
    members: Member[] = [
        { name: 'Amy Elsner', code: 'AE', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png' },
        { name: 'Anna Fali', code: 'AF', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/annafali.png' },
        { name: 'Asiya Javayant', code: 'AJ', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/asiyajavayant.png' },
        { name: 'Bernardo Dominic', code: 'BD', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/bernardodominic.png' },
        { name: 'Elwin Sharvill', code: 'ES', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/elwinsharvill.png' },
        { name: 'Ioni Bowcher', code: 'IB', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/ionibowcher.png' },
        { name: 'Ivan Magalhaes', code: 'IM', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/ivanmagalhaes.png' },
        { name: 'Stephen Shaw', code: 'SS', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/stephenshaw.png' }
    ];

    selected: string[] = [];

    getFirstName(code: string): string {
        const member = this.members.find((m) => m.code === code);

        return member ? member.name.split(' ')[0] : code;
    }

    getAvatar(code: string): string {
        return this.members.find((m) => m.code === code)?.avatar ?? '';
    }

    removeItem(event: MouseEvent, code: string) {
        event.stopPropagation();
        this.selected = this.selected.filter((c) => c !== code);
    }
}
```

## Clear

When showClear is enabled, a clear icon is displayed to clear the value.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { TagModule } from 'primeng/tag';

interface Category {
    label: string;
    value: string;
    count: number;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="categories" [(ngModel)]="selectedCategory" optionLabel="label" placeholder="Category" class="w-full md:w-56" [showClear]="true">
                <ng-template let-category #item>
                    <div class="flex items-center justify-between w-full">
                        <span>{{ category.label }}</span>
                        <p-tag [value]="'' + category.count" severity="secondary" [rounded]="true" />
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, TagModule, FormsModule]
})
export class SelectClearIconDemo {
    categories: Category[] = [
        { label: 'Electronics', value: 'electronics', count: 1247 },
        { label: 'Clothing', value: 'clothing', count: 856 },
        { label: 'Garden', value: 'home', count: 634 },
        { label: 'Sports', value: 'sports', count: 421 },
        { label: 'Books', value: 'books', count: 2103 },
        { label: 'Toys', value: 'toys', count: 312 }
    ];

    selectedCategory: Category | undefined;
}
```

## customfilter-doc

Custom filter can be applied with the filterTemplate .

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { Country } from '@/domain/customer';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="countries" [(ngModel)]="selectedCountry" optionLabel="name" [filter]="true" filterBy="name" [showClear]="true" placeholder="Select a Country">
                <ng-template #filter let-options="options">
                    <div class="flex gap-1">
                        <p-inputgroup (click)="$event.stopPropagation()">
                            <p-inputgroup-addon><i class="pi pi-search"></i></p-inputgroup-addon>
                            <input type="text" pInputText placeholder="Filter" [(ngModel)]="filterValue" (keyup)="customFilterFunction($event, options)" />
                        </p-inputgroup>
                        <button type="button" pButton iconOnly (click)="resetFunction(options)" severity="secondary"><i class="pi pi-times"></i></button>
                    </div>
                </ng-template>
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + selectedCountry.code.toLowerCase()" style="width: 18px" />
                        <div>{{ selectedOption.name }}</div>
                    </div>
                </ng-template>
                <ng-template #item let-country>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, InputGroupModule, InputGroupAddonModule, ButtonModule, InputTextModule, FormsModule]
})
export class SelectCustomFilterDemo implements OnInit {
    countries: City[] | undefined;

    selectedCountry: string | undefined;

    filterValue: string | undefined = '';

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

    resetFunction(options: SelectFilterOptions) {
        options.reset();
        this.filterValue = '';
    }

    customFilterFunction(event: KeyboardEvent, options: SelectFilterOptions) {
        options.filter(event);
    }
}
```

## customgroup-doc

Customize group headers with custom rendering using the group template.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { SelectItemGroup } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="groupedCities" [(ngModel)]="selectedCity" placeholder="Select a City" [group]="true" class="w-full md:w-56" [pt]="{ list: { class: 'p-0!' }, optionGroup: { class: 'border-b border-surface rounded-none' } }">
                <ng-template let-group #group>
                    <div class="flex items-center gap-2">
                        <span>{{ countryFlags[group.value] }}</span>
                        <span>{{ group.label }}</span>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectCustomGroupDemo {
    groupedCities: SelectItemGroup[] = [
        {
            label: 'Germany',
            value: 'DE',
            items: [
                { label: 'Berlin', value: 'Berlin' },
                { label: 'Frankfurt', value: 'Frankfurt' },
                { label: 'Hamburg', value: 'Hamburg' },
                { label: 'Munich', value: 'Munich' }
            ]
        },
        {
            label: 'USA',
            value: 'US',
            items: [
                { label: 'Chicago', value: 'Chicago' },
                { label: 'Los Angeles', value: 'Los Angeles' },
                { label: 'New York', value: 'New York' },
                { label: 'San Francisco', value: 'San Francisco' }
            ]
        },
        {
            label: 'Japan',
            value: 'JP',
            items: [
                { label: 'Kyoto', value: 'Kyoto' },
                { label: 'Osaka', value: 'Osaka' },
                { label: 'Tokyo', value: 'Tokyo' },
                { label: 'Yokohama', value: 'Yokohama' }
            ]
        }
    ];

    countryFlags: Record<string, string> = {
        DE: '🇩🇪',
        US: '🇺🇸',
        JP: '🇯🇵'
    };

    selectedCity: string | undefined;
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="cities" [(ngModel)]="selectedCity" placeholder="Select a City" optionLabel="name" [disabled]="true" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectDisabledDemo implements OnInit {
    cities: City[] | undefined;

    selectedCity: City | undefined;

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

## Editable

When editable is present, the input can also be entered with typing.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="cities" [(ngModel)]="selectedCity" placeholder="Select a City" [editable]="true" optionLabel="name" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectEditableDemo implements OnInit {
    cities: City[] | undefined;

    selectedCity: City | undefined;

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

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="cities" [(ngModel)]="selectedCity" variant="filled" optionLabel="name" placeholder="Select a City" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectFilledDemo implements OnInit {
    cities: City[];

    selectedCity: City | undefined;

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

Select provides built-in filtering that is enabled by adding the filter property.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { Country } from '@/domain/customer';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="countries" [(ngModel)]="selectedCountry" optionLabel="name" [filter]="true" filterBy="name" [showClear]="true" placeholder="Select a Country" class="w-full md:w-56">
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + selectedCountry.code.toLowerCase()" style="width: 18px" />
                        <div>{{ selectedOption.name }}</div>
                    </div>
                </ng-template>
                <ng-template let-country #item>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectFilterDemo implements OnInit {
    countries: Country[] | undefined;

    selectedCountry: Country | undefined;

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

## Float Label

A floating label appears on top of the input field when focused. Visit FloatLabel documentation for more information.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { FloatLabelModule } from 'primeng/floatlabel';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex flex-wrap justify-center items-end gap-4">
            <p-floatlabel class="w-full md:w-56">
                <p-select [(ngModel)]="value1" inputId="over_label" [options]="cities" optionLabel="name" class="w-full" />
                <label for="over_label">Over Label</label>
            </p-floatlabel>
            <p-floatlabel class="w-full md:w-56" variant="in">
                <p-select [(ngModel)]="value2" inputId="in_label" [options]="cities" optionLabel="name" class="w-full" variant="filled" />
                <label for="in_label">In Label</label>
            </p-floatlabel>
            <p-floatlabel class="w-full md:w-56" variant="on">
                <p-select [(ngModel)]="value3" inputId="on_label" [options]="cities" optionLabel="name" class="w-full" />
                <label for="on_label">On Label</label>
            </p-floatlabel>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FloatLabelModule, FormsModule]
})
export class SelectFloatLabelDemo implements OnInit {
    cities: City[] | undefined;

    value1: City | undefined;

    value2: City | undefined;

    value3: City | undefined;

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

## Fluid

The fluid prop makes the component take up the full width of its container when set to true.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <p-select [options]="cities" [(ngModel)]="selectedCity" optionLabel="name" placeholder="Select a City" fluid />
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectFluidDemo implements OnInit {
    cities: City[];

    selectedCity: City | undefined;

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

Customize keyboard and mouse focus behavior with autoOptionFocus , selectOnFocus , and focusOnHover properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { ButtonModule } from 'primeng/button';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2 justify-center">
                <button pButton (click)="autoOptionFocus = !autoOptionFocus" [severity]="autoOptionFocus ? 'primary' : 'secondary'" size="small">autoOptionFocus: {{ autoOptionFocus }}</button>
                <button pButton (click)="selectOnFocus = !selectOnFocus" [severity]="selectOnFocus ? 'primary' : 'secondary'" size="small">selectOnFocus: {{ selectOnFocus }}</button>
                <button pButton (click)="focusOnHover = !focusOnHover" [severity]="focusOnHover ? 'primary' : 'secondary'" size="small">focusOnHover: {{ focusOnHover }}</button>
            </div>
            <div class="flex justify-center">
                <p-select
                    [options]="cities"
                    [(ngModel)]="selectedCity"
                    optionLabel="name"
                    optionValue="code"
                    [autoOptionFocus]="autoOptionFocus"
                    [selectOnFocus]="selectOnFocus"
                    [focusOnHover]="focusOnHover"
                    placeholder="Select a city"
                    class="w-full md:w-56"
                />
            </div>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, ButtonModule, FormsModule]
})
export class SelectFocusBehaviorDemo {
    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    selectedCity: string | undefined;

    autoOptionFocus: boolean = true;

    selectOnFocus: boolean = false;

    focusOnHover: boolean = true;
}
```

## Group

Options can be grouped when a nested data structures is provided.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { SelectItemGroup } from 'primeng/api';
import { Product } from '@/domain/product';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="jobCategories" [(ngModel)]="selectedPosition" placeholder="Select a position..." [group]="true" optionLabel="label" optionValue="value" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectGroupDemo {
    jobCategories: SelectItemGroup[] = [
        {
            label: 'Engineering',
            value: 'eng',
            items: [
                { label: 'Frontend Developer', value: 'frontend' },
                { label: 'Backend Developer', value: 'backend' },
                { label: 'Full Stack Developer', value: 'fullstack' },
                { label: 'DevOps Engineer', value: 'devops' },
                { label: 'QA Engineer', value: 'qa' }
            ]
        },
        {
            label: 'Design',
            value: 'design',
            items: [
                { label: 'UI Designer', value: 'ui' },
                { label: 'UX Designer', value: 'ux' },
                { label: 'Product Designer', value: 'product-design' },
                { label: 'Brand Designer', value: 'brand' }
            ]
        },
        {
            label: 'Product',
            value: 'product',
            items: [
                { label: 'Product Manager', value: 'pm' },
                { label: 'Product Owner', value: 'po' },
                { label: 'Business Analyst', value: 'ba' }
            ]
        },
        {
            label: 'Marketing',
            value: 'marketing',
            items: [
                { label: 'Growth Manager', value: 'growth' },
                { label: 'Content Strategist', value: 'content' },
                { label: 'SEO Specialist', value: 'seo' }
            ]
        }
    ];

    selectedPosition: string | undefined;
}
```

## groupcheckboxfilter-doc

Combine grouped options, checkbox selection, and a filter to create a rich multi-select experience.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { SelectModule } from 'primeng/select';
import { ButtonModule } from 'primeng/button';
import { SelectItemGroup } from 'primeng/api';
import { Tag } from '@primeicons/angular/tag';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select
                [options]="labelGroups"
                [(ngModel)]="selected"
                [multiple]="true"
                [group]="true"
                [filter]="true"
                filterBy="label"
                filterPlaceholder="Search..."
                optionLabel="label"
                optionValue="value"
                placeholder="Select labels"
                class="w-full md:w-64 border-0! bg-transparent! shadow-none!"
                [pt]="{ dropdown: { class: 'hidden!' } }"
            >
                <ng-template #header>
                    <div class="flex items-center gap-1 px-3 pt-2">
                        <button pButton variant="text" size="small" (click)="selectAll()">Select All</button>
                        <button pButton variant="text" size="small" severity="secondary" (click)="selected = []">Clear</button>
                    </div>
                </ng-template>
                <ng-template #selectedItem>
                    <span class="inline-flex items-center gap-1 bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300 text-xs rounded-full py-1 px-3">
                        <svg data-p-icon="tag" />
                        <span class="font-semibold">{{ getTriggerLabel() }}</span>
                    </span>
                </ng-template>
                <ng-template let-group #group>
                    <span class="text-xs font-semibold text-surface-500 uppercase tracking-wide">{{ group.label }}</span>
                </ng-template>
                <ng-template let-item #item>
                    <div class="flex items-center gap-2">
                        <p-checkbox [ngModel]="isItemSelected(item)" [binary]="true" [tabindex]="-1" [readonly]="true" />
                        <span class="truncate">{{ item.label }}</span>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, SelectModule, ButtonModule, FormsModule, Tag]
})
export class SelectGroupCheckboxFilterDemo {
    labelGroups: SelectItemGroup[] = [
        {
            label: 'Critical',
            value: 'critical',
            items: [
                { label: 'Security', value: 'security' },
                { label: 'Performance', value: 'performance' }
            ]
        },
        {
            label: 'Optional',
            value: 'optional',
            items: [
                { label: 'Feature', value: 'feature' },
                { label: 'Other', value: 'other' }
            ]
        }
    ];

    selected: string[] = ['security'];

    get allItems() {
        return this.labelGroups.flatMap((g) => g.items);
    }

    getTriggerLabel(): string {
        if (this.selected.length === 0) return '';

        const first = this.allItems.find((i) => i.value === this.selected[0])?.label ?? this.selected[0];

        return this.selected.length > 1 ? `${first} +${this.selected.length - 1}` : first;
    }

    isItemSelected(item: { value: string }): boolean {
        return this.selected.includes(item.value);
    }

    selectAll() {
        this.selected = this.allItems.map((i) => i.value);
    }
}
```

## Ifta Label

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { IftaLabelModule } from 'primeng/iftalabel';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-iftalabel class="w-full md:w-56">
                <p-select [(ngModel)]="selectedCity" inputId="dd-city" [options]="cities" optionLabel="name" class="w-full" />
                <label for="dd-city">City</label>
            </p-iftalabel>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, IftaLabelModule, FormsModule]
})
export class SelectIftaLabelDemo implements OnInit {
    cities: City[] | undefined;

    selectedCity: City | undefined;

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

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center gap-4">
            <p-select [options]="cities" [(ngModel)]="selectedCity1" optionLabel="name" [showClear]="true" [invalid]="value1" placeholder="Select a City" class="w-full md:w-56" />
            <p-select [options]="cities" [(ngModel)]="selectedCity2" optionLabel="name" [showClear]="true" [invalid]="value2" placeholder="Select a City" class="w-full md:w-56" variant="filled" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectInvalidDemo {
    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    selectedCity1: City | undefined;

    selectedCity2: City | undefined;

    value1: boolean = true;

    value2: boolean = true;
}
```

## Lazy Virtual Scroll

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { SelectItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="items" [(ngModel)]="selectedItem" placeholder="Select Item" [virtualScroll]="true" [virtualScrollItemSize]="32" [virtualScrollOptions]="options" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectLazyVirtualScrollDemo {
    items: SelectItem[];

    selectedItem: string | undefined;

    loading: boolean = false;

    loadLazyTimeout: any = null;

    options: ScrollerOptions = {
        delay: 250,
        showLoader: true,
        lazy: true,
        onLazyLoad: this.onLazyLoad.bind(this)
    };

    constructor() {
        this.items = [];
        for (let i = 0; i < 10000; i++) {
            this.items.push({ label: 'Item ' + i, value: 'Item ' + i });
        }
    }

    onLazyLoad(event) {
        this.loading = true;

        if (this.loadLazyTimeout) {
            clearTimeout(this.loadLazyTimeout);
        }

        //imitate delay of a backend call
        this.loadLazyTimeout = setTimeout(
            () => {
                const { first, last } = event;
                const items = [...this.items];

                for (let i = first; i < last; i++) {
                    items[i] = { label: `Item #${i}`, value: i };
                }

                this.items = items;
                this.loading = false;
            },
            Math.random() * 1000 + 250
        );
    }
}
```

## Loading State

Loading state can be used loading property.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="cities" [(ngModel)]="selectedCity" [loading]="true" optionLabel="name" placeholder="Loading..." class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectLoadingStateDemo implements OnInit {
    cities: City[];

    selectedCity: City | undefined;

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

When multiple is enabled, multiple items can be selected. Use checkmark to display a check indicator and a custom #selectedItem template for the label.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface Topping {
    label: string;
    value: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="toppings" [(ngModel)]="selected" [multiple]="true" [checkmark]="true" optionLabel="label" optionValue="value" [showClear]="true" placeholder="Select Toppings" class="w-full md:w-56">
                <ng-template #selectedItem>
                    <span>{{ getLabel() }}</span>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectMultipleDemo {
    toppings: Topping[] = [
        { label: 'Pepperoni', value: 'pepperoni' },
        { label: 'Mushrooms', value: 'mushrooms' },
        { label: 'Onions', value: 'onions' },
        { label: 'Black Olives', value: 'olives' },
        { label: 'Green Peppers', value: 'peppers' },
        { label: 'Mozzarella', value: 'mozzarella' },
        { label: 'Basil', value: 'basil' },
        { label: 'Tomatoes', value: 'tomatoes' }
    ];

    selected: string[] = [];

    getLabel(): string {
        if (this.selected.length === 0) return '';

        const first = this.toppings.find((t) => t.value === this.selected[0])?.label ?? this.selected[0];

        return this.selected.length > 1 ? `${first} (+${this.selected.length - 1} more)` : first;
    }
}
```

## Custom Option

Custom content can be placed inside options using the item template.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { SelectModule } from 'primeng/select';
import { Product } from '@/domain/product';

interface TeamMember {
    id: number;
    name: string;
    role: string;
    avatar: string;
    statusClass: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="teamMembers" [(ngModel)]="assignee" optionLabel="name" placeholder="Select team member..." class="w-full md:w-56">
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex items-center gap-2">
                        <span [class]="'inline-block w-2 h-2 rounded-full ' + selectedOption.statusClass"></span>
                        <span>{{ selectedOption.name }}</span>
                    </div>
                </ng-template>
                <ng-template let-member #item>
                    <div class="flex items-center gap-3 py-1">
                        <div class="relative">
                            <p-avatar [label]="member.avatar" shape="circle" class="w-8 h-8" />
                            <span [class]="'absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-surface-0 dark:border-surface-900 ' + member.statusClass"></span>
                        </div>
                        <div class="flex flex-col">
                            <span class="font-medium">{{ member.name }}</span>
                            <span class="text-xs text-surface-500">{{ member.role }}</span>
                        </div>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, SelectModule, FormsModule]
})
export class SelectOptionDemo {
    teamMembers: TeamMember[] = [
        { id: 1, name: 'Sarah Chen', role: 'Engineering Lead', avatar: 'SC', statusClass: 'bg-amber-400' },
        { id: 2, name: 'Alex Rivera', role: 'Senior Developer', avatar: 'AR', statusClass: 'bg-green-400' },
        { id: 3, name: 'Jordan Kim', role: 'UX Designer', avatar: 'JK', statusClass: 'bg-amber-400' },
        { id: 4, name: 'Taylor Morgan', role: 'Product Manager', avatar: 'TM', statusClass: 'bg-zinc-400' },
        { id: 5, name: 'Morgan Lee', role: 'DevOps Engineer', avatar: 'ML', statusClass: 'bg-green-400' },
        { id: 6, name: 'Casey Jones', role: 'QA Engineer', avatar: 'CJ', statusClass: 'bg-red-400' }
    ];

    assignee: TeamMember | undefined;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface Language {
    label: string;
    value: string;
    code: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-select
                [options]="languages"
                [(ngModel)]="selectedLanguage"
                optionLabel="label"
                [filter]="true"
                filterBy="label"
                [checkmark]="true"
                placeholder="Select a language"
                class="w-full md:w-56"
                [pt]="{ option: { class: 'flex-row-reverse justify-between' } }"
            >
                <ng-template #selectedItem let-selectedOption>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + selectedOption.code" style="width: 18px; height: 12px" />
                        <span>{{ selectedOption.label }}</span>
                    </div>
                </ng-template>
                <ng-template let-language #item>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + language.code" style="width: 18px; height: 12px" />
                        <span>{{ language.label }}</span>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectPreviewDemo {
    languages: Language[] = [
        { label: 'English', value: 'en', code: 'us' },
        { label: 'Deutsch', value: 'de', code: 'de' },
        { label: 'Español', value: 'es', code: 'es' },
        { label: 'Français', value: 'fr', code: 'fr' },
        { label: 'Italiano', value: 'it', code: 'it' },
        { label: 'Português', value: 'pt', code: 'pt' },
        { label: 'Polski', value: 'pl', code: 'pl' },
        { label: 'Türkçe', value: 'tr', code: 'tr' },
        { label: 'Русский', value: 'ru', code: 'ru' },
        { label: '한국어', value: 'ko', code: 'kr' },
        { label: '日本語', value: 'ja', code: 'jp' },
        { label: '中文', value: 'zh', code: 'cn' }
    ];

    selectedLanguage: Language | undefined;
}
```

## reactiveforms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
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
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-select formControlName="city" [options]="cities" [invalid]="isInvalid('city')" optionLabel="name" placeholder="Select a City" class="w-full md:w-56" />
                    @if (isInvalid('city')) {
                        <p-message severity="error" size="small" variant="simple">City is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, MessageModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class SelectReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    exampleForm: FormGroup | undefined;

    formSubmitted: boolean = false;

    constructor() {
        this.exampleForm = this.fb.group({
            city: ['', Validators.required]
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

        return control?.invalid && (control.touched || this.formSubmitted);
    }
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { SelectModule } from 'primeng/select';
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
                    <p-select [formField]="exampleForm.city" [options]="cities" optionLabel="name" placeholder="Select a City" class="w-full md:w-56" />
                    @if (exampleForm.city().touched() && exampleForm.city().invalid()) {
                        @for (error of exampleForm.city().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, MessageModule, ToastModule, ButtonModule, FormField]
})
export class SelectSignalFormsDemo {
    messageService = inject(MessageService);

    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    model = signal<{ city: City | null }>({ city: null });

    exampleForm = form(this.model, (path) => {
        required(path.city, { when: ({ state }) => state.touched(), message: 'City is required.' });
    });

    onSubmit(event: Event) {
        event.preventDefault();
        submit(this.exampleForm, async () => {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        });
    }
}
```

## Sizes

Select provides small and large sizes as alternatives to the base.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';

interface City {
    name: string;
    code: string;
}

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <p-select [(ngModel)]="value1" [options]="cities" optionLabel="name" size="small" placeholder="Small" class="w-full md:w-56" />
            <p-select [(ngModel)]="value2" [options]="cities" optionLabel="name" placeholder="Normal" class="w-full md:w-56" />
            <p-select [(ngModel)]="value3" [options]="cities" optionLabel="name" size="large" placeholder="Large" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectSizesDemo implements OnInit {
    value1: City | undefined;

    value2: City | undefined;

    value3: City | undefined;

    cities: City[];

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

## Template

Both the selected option and the options list can be templated to provide customizated representation. Use selectedItem template to customize the selected label display and the item template to change the content of the options in the select panel. In addition when grouping is enabled, group template is available to customize the option groups. All templates get the option instance as the default local template variable.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="countries" [(ngModel)]="selectedCountry" optionLabel="name" placeholder="Select a country" class="w-full md:w-56">
                <ng-template #selectedItem let-selectedOption>
                    @if (selectedOption) {
                        <div class="flex items-center gap-2">
                            <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + selectedOption.code.toLowerCase()" style="width: 18px" />
                            <div>{{ selectedOption.name }}</div>
                        </div>
                    }
                </ng-template>
                <ng-template let-country #item>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
                <ng-template #dropdownicon>
                    <i class="pi pi-map"></i>
                </ng-template>
                <ng-template #header>
                    <div class="font-medium p-3">Available Countries</div>
                </ng-template>
                <ng-template #footer>
                    <div class="p-3">
                        <button type="button" pButton fluid severity="secondary" text size="small"><i class="pi pi-plus"></i> Add New</button>
                    </div>
                </ng-template>
            </p-select>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, ButtonModule, FormsModule]
})
export class SelectTemplateDemo implements OnInit {
    countries: any[] | undefined;

    selectedCountry: string | undefined;

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
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
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
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex flex-col gap-4 w-full sm:w-56">
                <div class="flex flex-col gap-1">
                    <p-select
                        #city="ngModel"
                        [(ngModel)]="selectedCity"
                        [options]="cities"
                        [invalid]="city.invalid && (city.touched || exampleForm.submitted)"
                        optionLabel="name"
                        name="city"
                        placeholder="Select a City"
                        class="w-full md:w-56"
                        required
                    />
                    @if (city.invalid && (city.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">City is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, MessageModule, ToastModule, ButtonModule, FormsModule]
})
export class SelectTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    cities: City[] = [
        { name: 'New York', code: 'NY' },
        { name: 'Rome', code: 'RM' },
        { name: 'London', code: 'LDN' },
        { name: 'Istanbul', code: 'IST' },
        { name: 'Paris', code: 'PRS' }
    ];

    selectedCity: City | undefined;

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
import { SelectModule } from 'primeng/select';
import { SelectItem } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-select [options]="items" [(ngModel)]="selectedItem" placeholder="Select Item" [virtualScroll]="true" [virtualScrollItemSize]="32" class="w-full md:w-56" />
        </div>
    `,
    standalone: true,
    imports: [SelectModule, FormsModule]
})
export class SelectVirtualScrollDemo {
    items: SelectItem[];

    selectedItem: string | undefined;

    constructor() {
        this.items = [];
        for (let i = 0; i < 10000; i++) {
            this.items.push({ label: 'Item ' + i, value: 'Item ' + i });
        }
    }
}
```

## Select

Select is used to choose an item from a collection of options.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, SelectPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| fluid | boolean \| undefined | false | Spans 100% width of the container when enabled. |
| variant | "filled" \| "outlined" \| undefined | 'outlined' | Specifies the input variant of the component. |
| size | "small" \| "large" \| undefined | undefined | Specifies the size of the component. |
| inputSize | number \| null \| undefined | undefined | Specifies the visible width of the input element in characters. |
| pattern | string \| readonly RegExp[] \| null \| undefined | undefined | Specifies the value must match the pattern. |
| min | any | undefined | The value must be greater than or equal to the value. |
| max | any | undefined | The value must be less than or equal to the value. |
| step | number \| null \| undefined | undefined | Unless the step is set to the any literal, the value must be min + an integral multiple of the step. |
| minlength | number \| null \| undefined | undefined | The number of characters (code points) must not be less than the value of the attribute, if non-empty. |
| maxlength | number \| null \| undefined | undefined | The number of characters (code points) must not exceed the value of the attribute. |
| id | string \| undefined | - | Unique identifier of the component |
| scrollHeight | string | - | Height of the viewport in pixels, a scrollbar is defined if height of list exceeds this value. |
| filter | boolean \| undefined | - | When specified, displays an input field to filter the items on keyup. |
| panelStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the overlay panel element. |
| panelStyleClass | string \| undefined | - | Style class of the overlay panel element. |
| readonly | boolean \| undefined | - | When present, it specifies that the component cannot be edited. |
| editable | boolean \| undefined | - | When present, custom value instead of predefined options can be entered using the editable input field. |
| tabindex | number | - | Index of the element in tabbing order. |
| placeholder | string \| undefined | - | Default text to display when no option is selected. |
| loadingIcon | string \| undefined | - | Icon to display in loading state. |
| filterPlaceholder | string \| undefined | - | Placeholder text to show when filter input is empty. |
| filterLocale | string \| undefined | - | Locale to use in filtering. The default locale is the host environment's current locale. |
| inputId | string \| undefined | - | Identifier of the accessible input element. |
| dataKey | string \| undefined | - | A property to uniquely identify a value in options. |
| filterBy | string \| undefined | - | When filtering is enabled, filterBy decides which field or fields (comma separated) to search against. |
| filterFields | any[] \| undefined | - | Fields used when filtering the options, defaults to optionLabel. |
| autofocus | boolean \| undefined | - | When present, it specifies that the component should automatically get focus on load. |
| resetFilterOnHide | boolean | - | Clears the filter value when hiding the select. |
| checkmark | boolean | - | Whether the selected option will be shown with a check mark. |
| dropdownIcon | string \| undefined | - | Icon class of the select icon. |
| loading | boolean | - | Whether the select is in loading state. |
| optionLabel | string \| undefined | - | Name of the label field of an option. |
| optionValue | string \| undefined | - | Name of the value field of an option. |
| optionDisabled | string \| ((item: any) => boolean) \| undefined | - | Name of the disabled field of an option or function to determine disabled state. |
| optionGroupLabel | string | - | Name of the label field of an option group. |
| optionGroupChildren | string | - | Name of the options field of an option group. |
| group | boolean \| undefined | - | Whether to display options as grouped when nested options are provided. |
| showClear | boolean \| undefined | - | When enabled, a clear icon is displayed to clear the value. |
| emptyFilterMessage | string | - | Text to display when filtering does not return any results. Defaults to global value in i18n translation configuration. |
| emptyMessage | string | - | Text to display when there is no data. Defaults to global value in i18n translation configuration. |
| lazy | boolean | - | Defines if data is loaded and interacted with in lazy manner. |
| virtualScroll | boolean \| undefined | - | Whether the data should be loaded on demand during scroll. |
| virtualScrollItemSize | number \| undefined | - | Height of an item in the list for VirtualScrolling. |
| virtualScrollOptions | ScrollerOptions \| undefined | - | Whether to use the scroller feature. The properties of scroller component can be used like an object in it. |
| overlayOptions | OverlayOptions \| undefined | - | Whether to use overlay API feature. The properties of overlay API can be used like an object in it. |
| ariaFilterLabel | string \| undefined | - | Defines a string that labels the filter input. |
| ariaLabel | string \| undefined | - | Used to define a aria label attribute the current element. |
| ariaLabelledBy | string \| undefined | - | Establishes relationships between the component and label(s) where its value should be one or more element IDs. |
| filterMatchMode | "startsWith" \| "contains" \| "notContains" \| "endsWith" \| "equals" \| "notEquals" \| "in" \| "between" \| "lt" \| "lte" \| "gt" \| "gte" \| "is" \| "isNot" \| "before" \| "after" \| "dateIs" \| "dateIsNot" \| "dateBefore" \| "dateAfter" | - | Defines how the items are filtered. |
| tooltip | string | - | Advisory information to display in a tooltip on hover. |
| tooltipPosition | "right" \| "left" \| "top" \| "bottom" | - | Position of the tooltip. |
| tooltipPositionStyle | string | - | Type of CSS position. |
| tooltipStyleClass | string \| undefined | - | Style class of the tooltip. |
| focusOnHover | boolean | - | Fields used when filtering the options, defaults to optionLabel. |
| selectOnFocus | boolean | - | Determines if the option will be selected on focus. |
| multiple | boolean | - | When enabled, allows multiple items to be selected. |
| autoOptionFocus | boolean | - | Whether to focus on the first visible or selected element when the overlay panel is shown. |
| autofocusFilter | boolean | - | Applies focus to the filter element when the overlay is shown. |
| filterValue | string \| null \| undefined | - | When specified, filter displays with this value. |
| options | any[] \| null \| undefined | - | An array of objects to display as the available options. |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'self' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| motionOptions | MotionOptions \| undefined | - | The motion options. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onChange | event: SelectChangeEvent | Callback to invoke when value of select changes. |
| onFilter | event: SelectFilterEvent | Callback to invoke when data is filtered. |
| onFocus | event: Event | Callback to invoke when select gets focus. |
| onBlur | event: Event | Callback to invoke when select loses focus. |
| onClick | event: MouseEvent | Callback to invoke when component is clicked. |
| onShow | event: MotionEvent & AnimationEvent | Callback to invoke when select overlay gets visible. |
| onHide | event: MotionEvent & AnimationEvent | Callback to invoke when select overlay gets hidden. |
| onClear | event: Event | Callback to invoke when select clears the value. |
| onLazyLoad | event: SelectLazyLoadEvent | Callback to invoke in lazy mode to load new data. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<SelectItemTemplateContext<any>> \| undefined | Custom item template. |
| group | TemplateRef<SelectGroupTemplateContext<any>> \| undefined | Custom group template. |
| loader | TemplateRef<SelectLoaderTemplateContext> \| undefined | Custom loader template. |
| selecteditem | TemplateRef<SelectSelectedItemTemplateContext<any>> \| undefined | Custom selected item template. |
| header | TemplateRef<SelectHeaderTemplateContext> \| undefined | Custom header template. |
| filter | TemplateRef<SelectFilterTemplateContext> \| undefined | Custom filter template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| emptyfilter | TemplateRef<void> \| undefined | Custom empty filter template. |
| empty | TemplateRef<void> \| undefined | Custom empty template. |
| dropdownicon | TemplateRef<SelectIconTemplateContext> \| undefined | Custom dropdown icon template. |
| loadingicon | TemplateRef<void> \| undefined | Custom loading icon template. |
| clearicon | TemplateRef<SelectIconTemplateContext> \| undefined | Custom clear icon template. |
| filtericon | TemplateRef<void> \| undefined | Custom filter icon template. |
| onicon | TemplateRef<void> \| undefined | Custom on icon template. |
| officon | TemplateRef<void> \| undefined | Custom off icon template. |
| cancelicon | TemplateRef<void> \| undefined | Custom cancel icon template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| resetFilter |  | void | Callback to invoke on filter reset. |
| show | isFocus: any | void | Displays the panel. |
| hide | isFocus: any | void | Hides the panel. |
| focus |  | void | Applies focus. |
| clear | event: Event | void | Clears the model. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the root's DOM element. |
| label | PassThroughOption<HTMLInputElement \| HTMLSpanElement, I> | Used to pass attributes to the label's DOM element. |
| clearIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the clear icon's DOM element. |
| dropdown | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the dropdown's DOM element. |
| loadingIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the loading icon's DOM element. |
| dropdownIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the dropdown icon's DOM element. |
| pcOverlay | OverlayPassThrough | Used to pass attributes to the Overlay component. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| pcFilterContainer | IconFieldPassThrough | Used to pass attributes to the filter container's DOM element. |
| pcFilter | InputTextPassThrough | Used to pass attributes to the filter input's DOM element. |
| pcFilterIconContainer | InputIconPassThrough | Used to pass attributes to the filter icon container's DOM element. |
| filterIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the filter icon's DOM element. |
| listContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the list container's DOM element. |
| virtualScroller | VirtualScrollerPassThrough | Used to pass attributes to the VirtualScroller component. |
| list | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the list's DOM element. |
| optionGroup | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the option group's DOM element. |
| optionGroupLabel | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the option group label's DOM element. |
| option | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the option's DOM element. |
| optionCheckIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the option check icon's DOM element. |
| optionBlankIcon | PassThroughOption<HTMLElement, I> | Used to pass attributes to the option blank icon's DOM element. |
| optionLabel | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the option label's DOM element. |
| emptyMessage | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the empty message's DOM element. |
| hiddenFirstFocusableEl | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden first focusable element's DOM element. |
| hiddenFilterResult | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden filter result's DOM element. |
| hiddenEmptyMessage | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden empty message's DOM element. |
| hiddenSelectedMessage | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden selected message's DOM element. |
| hiddenLastFocusableEl | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the hidden last focusable element's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-select | Class name of the root element |
| p-select-label | Class name of the label element |
| p-select-clear-icon | Class name of the clear icon element |
| p-select-dropdown | Class name of the dropdown element |
| p-select-loading-icon | Class name of the loadingicon element |
| p-select-dropdown-icon | Class name of the dropdown icon element |
| p-select-overlay | Class name of the overlay element |
| p-select-header | Class name of the header element |
| p-select-filter | Class name of the filter element |
| p-select-list-container | Class name of the list container element |
| p-select-list | Class name of the list element |
| p-select-option-group | Class name of the option group element |
| p-select-option-group-label | Class name of the option group label element |
| p-select-option | Class name of the option element |
| p-select-option-label | Class name of the option label element |
| p-select-option-check-icon | Class name of the option check icon element |
| p-select-option-blank-icon | Class name of the option blank icon element |
| p-select-empty-message | Class name of the empty message element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| select.background | --p-select-background | Background of root |
| select.disabled.background | --p-select-disabled-background | Disabled background of root |
| select.filled.background | --p-select-filled-background | Filled background of root |
| select.filled.hover.background | --p-select-filled-hover-background | Filled hover background of root |
| select.filled.focus.background | --p-select-filled-focus-background | Filled focus background of root |
| select.border.color | --p-select-border-color | Border color of root |
| select.hover.border.color | --p-select-hover-border-color | Hover border color of root |
| select.focus.border.color | --p-select-focus-border-color | Focus border color of root |
| select.invalid.border.color | --p-select-invalid-border-color | Invalid border color of root |
| select.color | --p-select-color | Color of root |
| select.disabled.color | --p-select-disabled-color | Disabled color of root |
| select.placeholder.color | --p-select-placeholder-color | Placeholder color of root |
| select.invalid.placeholder.color | --p-select-invalid-placeholder-color | Invalid placeholder color of root |
| select.shadow | --p-select-shadow | Shadow of root |
| select.padding.x | --p-select-padding-x | Padding x of root |
| select.padding.y | --p-select-padding-y | Padding y of root |
| select.border.radius | --p-select-border-radius | Border radius of root |
| select.focus.ring.width | --p-select-focus-ring-width | Focus ring width of root |
| select.focus.ring.style | --p-select-focus-ring-style | Focus ring style of root |
| select.focus.ring.color | --p-select-focus-ring-color | Focus ring color of root |
| select.focus.ring.offset | --p-select-focus-ring-offset | Focus ring offset of root |
| select.focus.ring.shadow | --p-select-focus-ring-shadow | Focus ring shadow of root |
| select.transition.duration | --p-select-transition-duration | Transition duration of root |
| select.sm.font.size | --p-select-sm-font-size | Sm font size of root |
| select.sm.padding.x | --p-select-sm-padding-x | Sm padding x of root |
| select.sm.padding.y | --p-select-sm-padding-y | Sm padding y of root |
| select.lg.font.size | --p-select-lg-font-size | Lg font size of root |
| select.lg.padding.x | --p-select-lg-padding-x | Lg padding x of root |
| select.lg.padding.y | --p-select-lg-padding-y | Lg padding y of root |
| select.font.weight | --p-select-font-weight | Font weight of root |
| select.font.size | --p-select-font-size | Font size of root |
| select.dropdown.width | --p-select-dropdown-width | Width of dropdown |
| select.dropdown.color | --p-select-dropdown-color | Color of dropdown |
| select.overlay.background | --p-select-overlay-background | Background of overlay |
| select.overlay.border.color | --p-select-overlay-border-color | Border color of overlay |
| select.overlay.border.radius | --p-select-overlay-border-radius | Border radius of overlay |
| select.overlay.color | --p-select-overlay-color | Color of overlay |
| select.overlay.shadow | --p-select-overlay-shadow | Shadow of overlay |
| select.list.padding | --p-select-list-padding | Padding of list |
| select.list.gap | --p-select-list-gap | Gap of list |
| select.list.header.padding | --p-select-list-header-padding | Header padding of list |
| select.option.focus.background | --p-select-option-focus-background | Focus background of option |
| select.option.selected.background | --p-select-option-selected-background | Selected background of option |
| select.option.selected.focus.background | --p-select-option-selected-focus-background | Selected focus background of option |
| select.option.color | --p-select-option-color | Color of option |
| select.option.focus.color | --p-select-option-focus-color | Focus color of option |
| select.option.selected.color | --p-select-option-selected-color | Selected color of option |
| select.option.selected.focus.color | --p-select-option-selected-focus-color | Selected focus color of option |
| select.option.selected.font.weight | --p-select-option-selected-font-weight | Font weight of a selected option |
| select.option.padding | --p-select-option-padding | Padding of option |
| select.option.border.radius | --p-select-option-border-radius | Border radius of option |
| select.option.font.weight | --p-select-option-font-weight | Font weight of option |
| select.option.font.size | --p-select-option-font-size | Font size of option |
| select.option.group.background | --p-select-option-group-background | Background of option group |
| select.option.group.color | --p-select-option-group-color | Color of option group |
| select.option.group.font.weight | --p-select-option-group-font-weight | Font weight of option group |
| select.option.group.font.size | --p-select-option-group-font-size | Font size of option group |
| select.option.group.padding | --p-select-option-group-padding | Padding of option group |
| select.clear.icon.color | --p-select-clear-icon-color | Color of clear icon |
| select.checkmark.color | --p-select-checkmark-color | Color of checkmark |
| select.checkmark.gutter.start | --p-select-checkmark-gutter-start | Gutter start of checkmark |
| select.checkmark.gutter.end | --p-select-checkmark-gutter-end | Gutter end of checkmark |
| select.empty.message.padding | --p-select-empty-message-padding | Padding of empty message |
