import { NgModule } from '@angular/core'
import { BrowserModule } from '@angular/platform-browser'
import { AppRoutingModule } from './app-routing.module'
import { AppComponent } from './app.component'
import { HttpClientModule } from '@angular/common/http'
import { FormsModule } from '@angular/forms'
import { NgxEchartsModule } from 'ngx-echarts'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'
import { MatLegacySelectModule as MatSelectModule } from '@angular/material/legacy-select'
import { MatLegacyFormFieldModule as MatFormFieldModule } from '@angular/material/legacy-form-field'
import { MatLegacyInputModule as MatInputModule } from '@angular/material/legacy-input'
import { MatLegacyTableModule as MatTableModule } from '@angular/material/legacy-table'
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome'

@NgModule({
   declarations: [
      AppComponent
   ],
   imports: [
      BrowserModule,
      AppRoutingModule,
      HttpClientModule,
      FormsModule,
      MatSelectModule,
      MatFormFieldModule,
      MatInputModule,
      MatTableModule,
      FontAwesomeModule,
      NgxEchartsModule.forRoot({
         /**
          * This will import all modules from echarts.
          * If you only need custom modules,
          * please refer to [Custom Build] section.
          */
         echarts: () => import('echarts'), // or import('./path-to-my-custom-echarts')
      }),
      BrowserAnimationsModule
   ],
   providers: [],
   bootstrap: [AppComponent]
})
export class AppModule { }
