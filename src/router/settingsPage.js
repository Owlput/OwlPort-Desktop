export default  {
    path:'/settings',
    component:()=>import("../layouts/Settings.vue"),
    children:[
        {
            path:"certificates",
            component:()=>import("../pages/settings/CertsAndKeys.vue")
        },
        {
          path:"general",
          component:()=>import("../pages/settings/General.vue")
      },
        {
            path:"",
            component:()=>import("../pages/settings/General.vue")
        }
    ]
}
