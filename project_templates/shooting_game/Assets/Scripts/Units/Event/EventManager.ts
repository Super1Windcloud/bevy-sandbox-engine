/**
 * 表示事件监听器函数的签名
 * 事件监听器接受一个类型为 'any' 的参数，并可以返回一个 'any' 类型的值
 *
 * @example
 * function myListener(data: any): any {
 *   console.log(`接收到的数据: ${data}`);
 *   return `处理后的数据: ${data}`;
 * }
 *
 * @type {(data: any) => any}
 */
type EventListener = (data: any) => any;

/**
 * EventManager类提供了一个事件管理系统，允许订阅、取消订阅和发布事件。
 * 
 * 使用这个类，可以创建一个事件中心，允许多个组件或对象在某个事件发生时得到通知。
 * 
 * 示例:
 * 1. 订阅事件:
 *    manager.Subscribe("eventName", (data) => { console.log(data); });
 * 
 * 2. 发布事件:
 *    manager.Publish("eventName", { message: "Hello, World!" });
 * 
 * 3. 取消订阅事件:
 *    manager.UnSubscribe("eventName", callbackFunction);
 * 
 * @property _eventListeners - 存储所有事件监听器的私有Map。
 * 
 * @class
 * @property {Map<string, { listener: EventListener; context: any | null }[]>} _eventListeners - 保存所有事件监听器的内部映射。
 * @method Subscribe - 订阅特定的事件。
 * @method UnSubscribe - 取消订阅特定的事件。
 * @method Publish - 发布一个事件，并调用所有订阅的监听器。
 */
class EventManager
{
	// 事件监听器的 Map
	private _eventListeners: Map<string, { listener: EventListener; context: any | null }[]> = new Map();

	/**
	 * 订阅事件的监听器
	 * @param eventName - 要订阅的事件的名称
	 * @param listener - 事件发布时要调用的函数
	 */
	public Subscribe(eventName: string, listener: EventListener, context: any = null): void
	{
		// 检查事件列表中是否已有指定的事件名称
		if (!this._eventListeners.has(eventName))
		{
			this._eventListeners.set(eventName, []);
		}

		// 获取 EventListener 数组
		// '!' 是非空断言，确保此值不是 'null' 或 'undefined'
		const array = this._eventListeners.get(eventName)!;
		array.push({ listener, context });
	}

	/**
	 * 从事件中取消订阅监听器
	 * @param eventName - 要取消订阅的事件的名称
	 * @param listener - 从事件中要删除的函数
	 */
	public UnSubscribe(eventName: string, listener: EventListener): void
	{
		if (this._eventListeners.has(eventName))
		{
			const array = this._eventListeners.get(eventName)!;
			const index = array.findIndex(item => item.listener === listener);

			if (index !== -1)
			{
				array.splice(index, 1);
			}
		}
	}

	/**
	 * 发布一个事件，调用所有订阅的监听器
	 * @param eventName - 要发布的事件的名称
	 * @param data - 要传递给监听器的数据
	 * @returns 包含监听器返回值的 Map，由监听器函数索引
	 */
	public Publish(eventName: string, data: any): void
	{
		const listeners = this._eventListeners.get(eventName);
		if (listeners)
		{
			for (const { listener, context } of listeners)
			{
				if (context)
				{
					listener.call(context, data);
				} else
				{
					listener(data);
				}
			}
		}
	}
}