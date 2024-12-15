import { render } from 'preact'
import { Provider } from 'react-redux'
import { store } from './store'
import './index.css'
import { App } from './app'

render(
    <Provider store={store}>
        <App />
    </Provider>,
    document.getElementById('app')
)
