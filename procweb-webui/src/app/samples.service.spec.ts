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
