PORT = 8080
require "http/server"
require "json"

module Main
  VERSION = "0.1.0"

  class Entry
    def initialize(description : String, urgency : Int32, area : String)
      @description = description
      @urgency = urgency
      @area = area
    end

    def description
      return @description
    end

    def urgency
      return @urgency
    end

    def area
      return @area
    end
  end

  class Dictionary
    def initialize
      @dictionary = Hash(Int32, Entry).new
    end

    def add_word(code : Int32, definition : Entry)
      @dictionary[code] = definition
    end

    def remove(code : Int32)
      @dictionary.delete(code)
    end

    def get_entry(code : Int32)
      return @dictionary[code]?
    end
  end

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
    def initialize
      @errors = Hash(Int32, Error).new
    end

    def add_error(number : Int32, error : Error)
      @errors[number] = error
    end

    def remove(number : Int32)
      if @errors[number].nil? #known issue -> can't recognize when error is real but not presently active?
        return nil
      end
      @errors.delete(number)
      return number
    end

    def make_json
      return JSON.build do |json|
        # json.field("errors") do
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
      end
    end
  end

  error_dictionary = Dictionary.new
  # set up error 1
  error_1 = Entry.new(description: "overcurrent", urgency: 0, area: "BMS")
  error_dictionary.add_word(1, error_1)
  error_2 = Entry.new(description: "overtemp", urgency: 0, area: "Motor")
  error_dictionary.add_word(2, error_2)
  error_3 = Entry.new(description: "overvoltage", urgency: 0, area: "BMS")
  error_dictionary.add_word(3, error_3)

  data = Data.new
  server = HTTP::Server.new do |context|
    current_path = context.request.path.lchop
    command = current_path.split("/").first # this will need to be fixed when adding the set command
    case (command)
    when "version"
      context.response.content_type = "text/plain"
      context.response.print VERSION
    when "get"
      context.response.content_type = "application/json"
      context.response.print data.make_json
    when "add"
      input_code = current_path.split("/").last.to_i
      error = error_dictionary.get_entry(input_code)
      if error.nil?
        context.response.content_type = "text/plain"
        context.response.print "Invalid"
      else
        data.add_error(input_code, Error.new(description: error.description, time: Time.utc.to_unix, urgency: error.urgency, area: error.area))
        context.response.content_type = "text/plain"
        context.response.print "#{input_code}"
      end
    when "remove"
      input_code = current_path.split("/").last.to_i
      removed = data.remove(input_code)
      if removed.nil?
        context.response.content_type = "text/plain"
        context.response.print "Invalid"
      else
        context.response.content_type = "text/plain"
        context.response.print "#{removed}"
      end
    else
      context.response.print "Unexpected command: #{command}"
    end
  end

  address = server.bind_tcp PORT
  puts "Listening on http://#{address}"
  server.listen
end
