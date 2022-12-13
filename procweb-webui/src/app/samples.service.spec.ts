/*
 * This file is part of procweb.
 *
 * Copyright (c) 2022 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/**
 * Author:  Luca Carlon
 * Date:    2022.12.13
 * Company: -
 */

import { TestBed } from '@angular/core/testing';
import { SamplesService } from './samples.service';

describe('SamplesService', () => {
  let service: SamplesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SamplesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
