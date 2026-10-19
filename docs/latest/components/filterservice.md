---
title: "FilterService - PrimeNG"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/filterservice.md"
description: "FilterService is a helper utility to filter collections against constraints."
---

# FilterService - PrimeNG

FilterService is a helper utility to filter collections against constraints.

## FilterService API

Following is the list of structural style classes, for theming classes visit theming page.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="doc-tablewrapper">
            <table class="doc-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Parameters</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>filter</td>
                        <td>
                            value[]: An array of values to filter<br />
                            fields[]: An array of properties in the value object<br />
                            filterValue: Filter value<br />
                            filterMatchMode: Constraint<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the property values of the given value collection matches the filter.</td>
                    </tr>
                    <tr>
                        <td>filters</td>
                        <td>-</td>
                        <td>Property to access constraints collection.</td>
                    </tr>
                    <tr>
                        <td>register</td>
                        <td>
                            name: string <br />
                            fn: Filter callback
                        </td>
                        <td>Registers a new constraint in filters.</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `,
    standalone: true,
    imports: []
})
export class FilterServiceApiDemo {}
```

## Built-in Constraints

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="doc-tablewrapper">
            <table class="doc-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Parameters</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>startsWith</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value starts with the filter value</td>
                    </tr>
                    <tr>
                        <td>contains</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value contains the filter value</td>
                    </tr>
                    <tr>
                        <td>endsWith</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value ends with the filter value</td>
                    </tr>
                    <tr>
                        <td>equals</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value equals the filter value</td>
                    </tr>
                    <tr>
                        <td>notEquals</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value does not equal the filter value</td>
                    </tr>
                    <tr>
                        <td>in</td>
                        <td>
                            value: Value to filter<br />
                            filter[]: An array of filter values<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value contains the filter value</td>
                    </tr>
                    <tr>
                        <td>lt</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value is less than the filter value</td>
                    </tr>
                    <tr>
                        <td>lte</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value is less than or equals to the filter value</td>
                    </tr>
                    <tr>
                        <td>gt</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value is greater than the filter value</td>
                    </tr>
                    <tr>
                        <td>gte</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value is greater than or equals to the filter value</td>
                    </tr>
                    <tr>
                        <td>is</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value equals the filter value, alias to equals</td>
                    </tr>
                    <tr>
                        <td>isNot</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the value does not equal the filter value, alias to notEquals.</td>
                    </tr>
                    <tr>
                        <td>before</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the date value is before the filter date.</td>
                    </tr>
                    <tr>
                        <td>after</td>
                        <td>
                            value: Value to filter<br />
                            filter: Filter value<br />
                            filterLocale: Locale to use in filtering
                        </td>
                        <td>Whether the date value is after the filter date.</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `,
    standalone: true,
    imports: []
})
export class FilterServiceBuiltInConstraintsDemo {}
```

## Custom Constraints

```typescript
import { Component } from '@angular/core';
import { FilterService } from 'primeng/api';

@Component({
    template: `
        <app-docsectiontext>
            <p>FilterService can be extended by adding new constraints using the <span>register</span> function.</p></app-docsectiontext
        >
    `,
    standalone: true,
    imports: []
})
export class FilterServiceCustomConstraintsDemo {}
```

## tableintegration-doc

A custom equals filter that checks for exact case sensitive value is registered and defined as a match mode of a column filter.

```typescript
import { Component, OnInit, inject, ChangeDetectorRef } from '@angular/core';
import { TableModule } from 'primeng/table';
import { CarService } from '@/service/carservice';
import { SelectItem, FilterService } from 'primeng/api';
import { Car } from '@/domain/car';
import { Product } from '@/domain/product';

@Component({
    template: `
        <p-table #dt [columns]="cols" [value]="cars" [paginator]="true" [rows]="10" [tableStyle]="{ 'min-width': '75rem' }">
            <ng-template #header let-columns>
                <tr>
                    @for (col of columns; track col.field) {
                        <th [style.width]="'25%'">{{ col.header }}</th>
                    }
                </tr>
                <tr>
                    @for (col of columns; track col.field) {
                        <th>
                            <p-column-filter type="text" [field]="col.field" [matchModeOptions]="matchModeOptions" [matchMode]="'custom-equals'" />
                        </th>
                    }
                </tr>
            </ng-template>
            <ng-template #body let-rowData let-columns="columns">
                <tr [pSelectableRow]="rowData">
                    @for (col of columns; track col.field) {
                        <td>{{ rowData[col.field] }}</td>
                    }
                </tr>
            </ng-template>
        </p-table>
    `,
    standalone: true,
    imports: [TableModule],
    providers: [CarService, FilterService]
})
export class FilterServiceTableIntegrationDemo implements OnInit {
    private carService = inject(CarService);

    private filterService = inject(FilterService);

    private cd = inject(ChangeDetectorRef);

    cars: Car[];

    cols: any[];

    matchModeOptions: SelectItem[];

    ngOnInit() {
        const customFilterName = 'custom-equals';
        this.filterService.register(customFilterName, (value, filter): boolean => {
            if (filter === undefined || filter === null || filter.trim() === '') {
                return true;
            }
            if (value === undefined || value === null) {
                return false;
            }
            return value.toString() === filter.toString();
        });
        this.cols = [
            { field: 'year', header: 'Year' },
            { field: 'brand', header: 'Brand' },
            { field: 'color', header: 'Color' },
            { field: 'vin', header: 'Vin' }
        ];
        this.matchModeOptions = [
            { label: 'Custom Equals', value: customFilterName },
            { label: 'Starts With', value: FilterMatchMode.STARTS_WITH },
            { label: 'Contains', value: FilterMatchMode.CONTAINS }
        ];
        this.carService.getCarsMedium().then((cars) => {
            this.cars = cars;
        });
    }
}
```
