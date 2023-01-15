# TODO: Write documentation for `Demo`
PORT = 8080
require "http/server"
require "json"
module Demo
  VERSION = "0.1.0"
  # initialized values of empty (always starts with )

  class Error
    def initialize(description : String, time : Int64, urgency : Int32, area : String)
      @description = description
      @time = time
      @urgency = urgency
      @area = area
    end
    def description
      return @description
    end
    def time
      return @time
    end
    def urgency
      return @urgency
    end
    def area
      return @area
    end
  end

  class Data
    def initialize()
      @errors = Hash(Int32, Error).new
    end
    def add_error(number : Int32, error : Error)
      @errors[number] = error
    end
    def remove(number : Int32)
      @errors.delete(number)
    end
    def make_json()
      return JSON.build do |json|
        #json.field("errors") do
          json.object do
            @errors.each do |number, error|
              json.field(number.to_s) do
                json.object do
                  json.field("description", error.description)
                  json.field("time", error.time)
                  json.field("urgency", error.urgency)
                  json.field("area", error.area)
                end
              end
            end
          end
        #end
        #json.field("timestamp", 123) #fix with Time library
      end
    end
  end

  data = Data.new
  data.add_error(123, Error.new(description: "overcurrent", time: 12134, urgency: 0, area: "BMS"))
  data.add_error(1, Error.new(description: "overcurrent", time: 12134, urgency: 0, area: "BMS"))
  data.add_error(23, Error.new(description: "overcurrent", time: 12134, urgency: 0, area: "BMS"))
  data.remove(1)


  server = HTTP::Server.new do |context|
    context.response.content_type = "text/plain"
    current_path = context.request.path.lchop
    command = current_path.split("/").first #this will need to be fixed when adding the set command
    case(command)

    when "version"
      context.response.print VERSION

    when "get"
      context.response.content_type = "application/json"
      context.response.print data.make_json

    when "set"
      context.response.content_type = "application/json"
      context.response.print "#{current_path}"
    else
      context.response.print "Unexpected command: #{command}"
    end


  end

  address = server.bind_tcp PORT
  puts "Listening on http://#{address}"
  server.listen
end
