import { NgModule } from '@angular/core'
import { BrowserModule } from '@angular/platform-browser'
import { AppRoutingModule } from './app-routing.module'
import { AppComponent } from './app.component'
import { HttpClientModule } from '@angular/common/http'
import { FormsModule } from '@angular/forms'
import { NgxEchartsModule } from 'ngx-echarts'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'
import { MatSelectModule } from '@angular/material/select'
import { MatFormFieldModule } from '@angular/material/form-field'
import { MatInputModule } from '@angular/material/input'
import { MatTableModule } from '@angular/material/table'
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
